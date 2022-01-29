use crate::client_builder::ClientBuilder;
use anyhow::Result;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoginQrCodeData {
    pub oauthKey: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLoginQrCodeApiResponse {
    pub code: u32,
    pub status: bool,
    pub ts: u64,
    pub data: GetLoginQrCodeData,
}

pub async fn get_login_qr_code_url(client: reqwest::Client) -> Result<String> {
    let get_login_url = "http://passport.bilibili.com/qrcode/getLoginUrl";
    let resp = client
        .get(get_login_url)
        .send()
        .await?
        .json::<GetLoginQrCodeApiResponse>()
        .await?;
    println!("{:#?}", resp.data.url);
    Ok(resp.data.url)
}
