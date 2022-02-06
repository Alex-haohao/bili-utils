use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayQueryResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "order_id")]
    pub order_id: String,
    pub mid: i64,
    pub platform: String,
    #[serde(rename = "item_id")]
    pub item_id: i64,
    #[serde(rename = "pay_id")]
    pub pay_id: String,
    pub state: String,
}
