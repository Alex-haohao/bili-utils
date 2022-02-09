extern crate rand;
use anyhow::{anyhow, Result};
use chrono::Utc;
use rand::Rng;
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
        .await;
    if let Ok(resp) = res {
        Ok(resp.data.now)
    } else {
        Ok(Utc::now().timestamp())
    }
}

pub fn random_id(PASSWORD_LEN: u32, long: bool) -> String {
    let mut charset: &[u8] = b"0123456789abcdef";
    if long {
        charset = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    }
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect();

    password
}

pub fn get_current_local_time() -> i64 {
    let local_time = Utc::now().timestamp();
    local_time
}
