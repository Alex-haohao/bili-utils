use crate::client::parse_cookies;
use crate::login::GetLoginStatusResponseData::DataOk;
use anyhow::Result;
use crossbeam_channel::tick;
use dialoguer::Confirm;
use qrcode::render::unicode;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{fs, process, thread};
use url::Url;

pub static UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 15_3 like Mac OS X) AppleWebKit/612.4.9.1.5 (KHTML, like Gecko) Mobile/21D49 BiliApp/65500100 os/ios model/iPad Pro 12.9-Inch 3G mobi_app/iphone_b build/65500100 osVer/15.3 network/2 channel/AppStore Buvid/Y556CB5651036FC351CAA1360C6FEB723795 c_locale/zh-Hans_CN s_locale/zh-Hans_CN sessionID/9a454e04 disable_rcmd/0";

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoginQrCodeData {
    pub oauthKey: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoginQrCodeApiResponse {
    pub code: i32,
    pub status: bool,
    pub ts: i64,
    pub data: GetLoginQrCodeData,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum GetLoginStatusResponseData {
    DataFail(i32),
    DataOk { url: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoginStatusResponse {
    pub status: bool,
    pub data: GetLoginStatusResponseData,
    pub message: Option<String>,
    pub ts: Option<i64>,
}

/*
得到登录二维码的url
*/
pub async fn get_login_prepare_response(client: &reqwest::Client) -> Result<(String, String)> {
    let get_login_url = "http://passport.bilibili.com/qrcode/getLoginUrl";
    let resp = client
        .get(get_login_url)
        .send()
        .await?
        .json::<GetLoginQrCodeApiResponse>()
        .await?;

    let code = QrCode::new(&resp.data.url)?;
    let qrcode = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    Ok((qrcode, resp.data.oauthKey))
}

pub async fn polling_login_info(
    client: &reqwest::Client,
    oauthKey: &String,
) -> Result<user_info_params> {
    let get_login_url = "http://passport.bilibili.com/qrcode/getLoginInfo";
    // 构造post_data
    let mut post_data = HashMap::new();
    post_data.insert("oauthKey", oauthKey);
    println!("{:?}", post_data);

    let start = Instant::now();
    // 每一秒轮询一次
    let ticker = tick(Duration::from_millis(1000));
    loop {
        let msg = ticker.recv().unwrap();
        println!("{:?} elapsed: {:?}", msg, start.elapsed());
        let resp = client
            .post(get_login_url)
            .form(&post_data)
            .send()
            .await?
            .json::<GetLoginStatusResponse>()
            .await?;

        if resp.status == true {
            if let DataOk { url } = resp.data {
                let url = Url::parse(&url)?;
                let cookies = handle_record_cookies(&url.query().expect("query"));
                return Ok(cookies);
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct user_info_params {
    pub DedeUserID: i32,           //UID
    pub DedeUserID__ckMd5: String, //UID_Decode
    pub Expires: i32,              //过期时间
    pub SESSDATA: String,          // Session
    pub bili_jct: String,          // JCT
    pub gourl: String,
}

// 解析用户信息 并写入文件
pub fn handle_record_cookies(params: &str) -> user_info_params {
    let parsed_params: user_info_params = qs::from_str(params).expect("parse params");

    // 删除过期的文件
    let remove = fs::remove_file("bili_info.txt");
    if let Err(e) = remove {};

    let mut f = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("bili_info.txt")
        .expect("Unable to open/create file");

    // 将用户信息写入文件
    f.write_all(
        serde_json::to_string(&parsed_params)
            .expect("serde_cookie")
            .as_bytes(),
    )
    .expect("Unable to write cookies to file");
    parsed_params
}

pub fn read_user_info_from_file() -> Result<user_info_params> {
    let foo = fs::read_to_string("bili_info.txt");
    match foo {
        Ok(s) => {
            let parsed_params: user_info_params = serde_json::from_str(&s).expect("parse params");
            Ok(parsed_params)
        }
        Err(e) => Err(e.into()),
    }
}

/**
 * 登录主流程：
 * 1. 获取二维码
 * 2. 开始轮询登录状态
 * 3. 获取cookies
 */
pub async fn login() -> Result<user_info_params> {
    let client = reqwest::ClientBuilder::new().user_agent(UA).build()?;
    let qrcode = get_login_prepare_response(&client).await?;
    let (qrcode, oauthKey) = qrcode;
    println!("{}", qrcode);
    // 开始轮询用户是否登录
    let cookies = polling_login_info(&client, &oauthKey).await?;
    Ok(cookies)
}

pub async fn check_login_status() -> Result<user_info_params> {
    let read_user_data = read_user_info_from_file();
    match read_user_data {
        // 如果本地有登录信息，则直接读取
        Ok(cookies) => Ok(cookies),
        Err(e) => {
            // 如果本地没有登录信息，则需要登录
            if Confirm::new()
                .with_prompt("没有发现登录状态，是否选择登录账号？")
                .interact()?
            {
                println!("请扫描二维码登录");
                let cookies = login().await?;
                println!("登录成功");
                return Ok(cookies);
            } else {
                println!("即将退出程序");
                std::process::exit(0);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavResponseData {
    isLogin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NavResponse {
    pub code: i32,
    pub data: NavResponseData,
    pub message: String,
}

/**
 * 测试登录状态/cookies有效性
 */
pub async fn test_login_status(cookies: user_info_params) -> Result<user_info_params> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent(UA)
        .build()?;
    let cookies_header = parse_cookies(&cookies);
    let url = "https://api.bilibili.com/x/web-interface/nav";
    let resp = client
        .get(url)
        .header("cookie", cookies_header)
        .send()
        .await?
        .json::<NavResponse>()
        .await;

    if let Ok(resp) = resp {
        if resp.data.isLogin {
            println!("登录成功");
        } else {
            if Confirm::new()
                .with_prompt("登录状态已失效，是否重新登录？")
                .interact()?
            {
                println!("请扫描二维码登录");
                let cookies = login().await?;
                println!("登录成功");
                return Ok(cookies);
            } else {
                println!("即将退出程序");
                std::process::exit(0);
            }
        }
    } else {
        process::exit(0);
    }

    Ok(cookies)
}
