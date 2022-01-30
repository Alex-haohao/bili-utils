#[macro_use]
extern crate serde_qs as qs;

use crate::login::{get_login_prepare_response, polling_login_info};
use anyhow::Result;
// 错误处理
use dialoguer::Confirm;
use reqwest;
// 网络请求
use std::io::Read; // 读取文件 // 用户交互

mod client_builder;
mod login;
mod ticker;

// UA string to pass to ClientBuilder.user_agent
static UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 15_3 like Mac OS X) AppleWebKit/612.4.9.1.5 (KHTML, like Gecko) Mobile/21D49 BiliApp/65500100 os/ios model/iPad Pro 12.9-Inch 3G mobi_app/iphone_b build/65500100 osVer/15.3 network/2 channel/AppStore Buvid/Y556CB5651036FC351CAA1360C6FEB723795 c_locale/zh-Hans_CN s_locale/zh-Hans_CN sessionID/9a454e04 disable_rcmd/0";

/**
 * 登录主流程：
 * 1. 获取二维码
 * 2. 开始轮询登录状态
 * 3. 查看登录信息
 */
pub async fn login() -> Result<(String, String)> {
    let client = reqwest::ClientBuilder::new().user_agent(UA).build()?;
    let qrcode = get_login_prepare_response(&client).await?;
    let (qrcode, oauthKey) = qrcode;
    println!("{}", qrcode);
    // 开始轮询用户是否登录
    polling_login_info(&client, &oauthKey).await;
    Ok((qrcode, oauthKey))
}

/**
 * 从用户本地读取cookies信息
 */
pub fn read_user_info_file() -> Result<String> {
    let mut file = std::fs::File::open("user_info.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut lines = contents.lines();
    let cookies = lines.next().expect("没有发现cookies");
    Ok(cookies.to_string())
}

pub async fn check_login_status() -> Result<String> {
    let cookies = read_user_info_file();
    println!("{:?}", cookies);
    match cookies {
        Ok(cookies) => {
            println!("cookies: {}", cookies);
            Ok(cookies)
        }
        Err(e) => {
            if Confirm::new()
                .with_prompt("没有发现登录状态，是否选择登录账号？")
                .interact()?
            {
                println!("请扫描二维码登录");
                let a = login().await;
            } else {
                println!("即将退出程序");
                std::process::exit(0);
            }
            Err(e)
        }
    }
}
