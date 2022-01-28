use crate::client_builder::ClientBuilder;
use anyhow::Result;
use reqwest;
use reqwest::{Error, Response};

mod client_builder;

pub async fn test() -> Result<Response> {
    let temp_client = reqwest::Client::new();
    // create default ClientBuilder
    let client_builder = ClientBuilder::default();
    // 设置自定义的client
    let client_builder = client_builder.set_reqwest_client(temp_client);
    let client_builder = client_builder.build();

    match client_builder {
        Ok(client) => {
            let res = client.get("https://www.baidu.com").send().await;
            return match res {
                Ok(response) => Ok(response),
                Err(e) => {
                    println!("{:?}", e);
                    Err(e.into())
                }
            };
        }
        Err(e) => Err(e),
    }
}
