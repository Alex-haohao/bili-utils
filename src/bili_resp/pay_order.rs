use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayOrderResp {
    pub code: Option<i64>,
    pub errno: i64,
    pub msg: String,
    pub show_msg: String,
    pub data: Option<Data>,
    pub success: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub trace_id: String,
    pub server_time: i64,
    pub customer_id: i64,
    pub order_id: String,
    pub tx_id: i64,
    pub pay_channel: String,
    pub device_type: i64,
    pub pay_channel_param: String,
    pub pay_channel_url: String,
    #[serde(rename = "queryOrderReqVO")]
    pub query_order_req_vo: QueryOrderReqVo,
    pub return_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOrderReqVo {
    pub trace_id: String,
    pub check_third_channel: String,
    pub customer_id: String,
    pub sign: String,
    pub sign_type: String,
    pub version: String,
    pub tx_ids: String,
    pub timestamp: String,
}
