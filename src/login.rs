use anyhow::Result;
use qrcode::render::unicode;
use qrcode::QrCode;
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

/*
得到登录二维码的url
*/
pub async fn get_login_prepare_response(client: reqwest::Client) -> Result<(String, String)> {
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

    println!("{:?}", resp.data);
    Ok((qrcode, resp.data.oauthKey))
}

pub async fn polling_login_info(client: reqwest::Client, oauthKey: String) {
    let get_login_url = "http://passport.bilibili.com/qrcode/getLoginInfo";
}
