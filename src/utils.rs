use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliDateResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub now: i64,
}

pub async fn get_bili_server_time() -> Result<i64> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.bilibili.com/x/report/click/now")
        .send()
        .await?
        .json::<BiliDateResp>()
        .await?;
    Ok(res.data.now)
}
