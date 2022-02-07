use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayChannelParam {
    pub customer_id: i64,
    pub customer_name: String,
    pub tx_id: String,
    pub order_id: String,
    pub pay_amout: i64,
    pub fee_type: String,
    pub product_id: String,
    pub order_create_time: String,
    pub no_bp_coupon: i64,
    pub platform_type: i64,
    pub mid: i64,
    pub remark: String,
    pub sign: String,
    pub timestamp: i64,
}
