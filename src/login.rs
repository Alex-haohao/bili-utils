use anyhow::Result;
use qrcode::render::unicode;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
////// ticker::tick
use crossbeam::select;
use crossbeam_channel::after;
use crossbeam_channel::tick;
use crossbeam_channel::unbounded;
use serde_json::Value;
use std::thread;
use std::time::{Duration, Instant};

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
    pub message: String,
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

pub async fn polling_login_info(client: &reqwest::Client, oauthKey: &String) -> Result<String> {
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

        if resp.status == true {}
    }
}
