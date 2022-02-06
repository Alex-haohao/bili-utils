use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayDataResp {
    pub customer_id: i64,
    pub device_type: String,
    pub notify_url: String,
    pub order_create_time: String,
    pub order_expire: String,
    pub order_id: String,
    pub original_amount: String,
    pub pay_amount: String,
    pub product_id: String,
    pub service_type: String,
    pub show_title: String,
    pub sign: String,
    pub sign_type: String,
    pub timestamp: String,
    pub trace_id: String,
    pub uid: String,
    pub version: String,
}
