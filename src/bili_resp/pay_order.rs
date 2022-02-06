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
    pub wx_id: String,
    pub tx_id: String,
    pub customer_id: i64,
    pub total_pay_bp: i64,
    pub pay_counpon: i64,
    pub bp_rate: f64,
    pub fee_type: String,
    pub pay_amount: i64,
    pub pay_time: i64,
    pub status: String,
    pub order_id: String,
}
