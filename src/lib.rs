use crate::client_builder::ClientBuilder;
use crate::login::get_login_qr_code_url;
use anyhow::Result;
use reqwest;

mod client_builder;
mod login;

pub async fn test() -> Result<String> {
    let temp_client = reqwest::Client::new();
    // create default ClientBuilder
    let client_builder = ClientBuilder::default();
    // 设置自定义的client
    let client_builder = client_builder.set_reqwest_client(temp_client);
    let client = client_builder.build()?;
    let qrcode = get_login_qr_code_url(client).await;
    qrcode
}
