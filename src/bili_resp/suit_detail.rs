use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuitDetailResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub item: Item,
    #[serde(rename = "sale_surplus")]
    pub sale_surplus: i64,
    #[serde(rename = "sale_left_time")]
    pub sale_left_time: i64,
    #[serde(rename = "unlock_items")]
    pub unlock_items: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "item_id")]
    pub item_id: i64,
    pub name: String,
    pub state: String,
    #[serde(rename = "tab_id")]
    pub tab_id: i64,
    #[serde(rename = "suit_item_id")]
    pub suit_item_id: i64,
    pub properties: Option<Properties>,
    #[serde(rename = "current_activity")]
    pub current_activity: Option<Value>,
    #[serde(rename = "current_sources")]
    pub current_sources: Option<Value>,
    #[serde(rename = "finish_sources")]
    pub finish_sources: Option<Value>,
    #[serde(rename = "sale_left_time")]
    pub sale_left_time: i64,
    #[serde(rename = "sale_time_end")]
    pub sale_time_end: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub desc: String,
    #[serde(rename = "fan_desc")]
    pub fan_desc: String,
    #[serde(rename = "fan_id")]
    pub fan_id: String,
    #[serde(rename = "fan_item_ids")]
    pub fan_item_ids: String,
    #[serde(rename = "fan_mid")]
    pub fan_mid: String,
    #[serde(rename = "fan_no_color")]
    pub fan_no_color: String,
    #[serde(rename = "fan_recommend_desc")]
    pub fan_recommend_desc: String,
    #[serde(rename = "fan_recommend_jump_type")]
    pub fan_recommend_jump_type: String,
    #[serde(rename = "fan_recommend_jump_value")]
    pub fan_recommend_jump_value: String,
    #[serde(rename = "fan_share_image")]
    pub fan_share_image: String,
    #[serde(rename = "image_cover")]
    pub image_cover: String,
    #[serde(rename = "image_cover_color")]
    pub image_cover_color: String,
    #[serde(rename = "image_cover_long")]
    pub image_cover_long: String,
    #[serde(rename = "image_desc")]
    pub image_desc: String,
    #[serde(rename = "is_hide")]
    pub is_hide: String,
    #[serde(rename = "item_id_card")]
    pub item_id_card: String,
    #[serde(rename = "item_id_emoji")]
    pub item_id_emoji: String,
    #[serde(rename = "item_id_thumbup")]
    pub item_id_thumbup: String,
    #[serde(rename = "rank_investor_show")]
    pub rank_investor_show: String,
    #[serde(rename = "realname_auth")]
    pub realname_auth: String,
    #[serde(rename = "sale_bp_forever_raw")]
    pub sale_bp_forever_raw: String,
    #[serde(rename = "sale_bp_pm_raw")]
    pub sale_bp_pm_raw: String,
    #[serde(rename = "sale_buy_num_limit")]
    pub sale_buy_num_limit: String,
    #[serde(rename = "sale_quantity")]
    pub sale_quantity: String,
    #[serde(rename = "sale_quantity_limit")]
    pub sale_quantity_limit: String,
    #[serde(rename = "sale_region_ip_limit")]
    pub sale_region_ip_limit: String,
    #[serde(rename = "sale_reserve_switch")]
    pub sale_reserve_switch: String,
    #[serde(rename = "sale_time_begin")]
    pub sale_time_begin: String,
    #[serde(rename = "sale_type")]
    pub sale_type: String,
    #[serde(rename = "suit_card_type")]
    pub suit_card_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
