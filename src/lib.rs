use crate::client_builder::ClientBuilder;
use reqwest;
use reqwest::Client as ReqwestClient;

mod client_builder;
mod error;

pub async fn test() {
    let temp_client = reqwest::Client::new();
    // create default ClientBuilder
    let client_builder = ClientBuilder::default();
    // 设置自定义的client
    let client_builder = client_builder.set_reqwest_client(temp_client);
    let client_builder = client_builder.build();

    if let Err(ref error) = client_builder {
        println!("{:?}", error);
    }

    if let Ok(ref client) = client_builder {
        let result = client.get("https://api.spotify.com/v1/search").send().await;
        println!("{:?}", result);
    }
}
