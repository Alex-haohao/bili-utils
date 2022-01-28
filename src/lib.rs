use reqwest;
use reqwest::Client as ReqwestClient;

mod client_builder;
mod error;

// tokio let's us use "async" on our main function
#[tokio::main]
pub async fn test() {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.spotify.com/v1/search")
        // confirm the request using send()
        .send()
        .await
        // the rest is the same!
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
}
