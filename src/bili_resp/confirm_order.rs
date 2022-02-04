use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmOrderResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Option<Data>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub state: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "pay_data")]
    pub pay_data: String,
}
