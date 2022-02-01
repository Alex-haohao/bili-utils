use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuitAllResp {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub category: Vec<Category>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub name: String,
    pub suits: Vec<Suit>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Suit {
    #[serde(rename = "item_id")]
    pub item_id: i64,
    pub name: String,
    pub state: String,
    #[serde(rename = "tab_id")]
    pub tab_id: i64,
    #[serde(rename = "suit_item_id")]
    pub suit_item_id: i64,
    pub properties: Properties,
    #[serde(rename = "current_activity")]
    pub current_activity: Option<CurrentActivity>,
    #[serde(rename = "current_sources")]
    pub current_sources: Value,
    #[serde(rename = "finish_sources")]
    pub finish_sources: Value,
    #[serde(rename = "sale_left_time")]
    pub sale_left_time: i64,
    #[serde(rename = "sale_time_end")]
    pub sale_time_end: i64,
    #[serde(rename = "sale_surplus")]
    pub sale_surplus: i64,
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
    pub fan_recommend_desc: Option<String>,
    #[serde(rename = "fan_recommend_jump_type")]
    pub fan_recommend_jump_type: Option<String>,
    #[serde(rename = "fan_recommend_jump_value")]
    pub fan_recommend_jump_value: Option<String>,
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
    pub is_hide: Option<String>,
    #[serde(rename = "item_id_card")]
    pub item_id_card: String,
    #[serde(rename = "item_id_emoji")]
    pub item_id_emoji: String,
    #[serde(rename = "item_id_pendant")]
    pub item_id_pendant: Option<String>,
    #[serde(rename = "item_id_thumbup")]
    pub item_id_thumbup: String,
    #[serde(rename = "rank_investor_show")]
    pub rank_investor_show: String,
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
    pub sale_region_ip_limit: Option<String>,
    #[serde(rename = "sale_reserve_switch")]
    pub sale_reserve_switch: String,
    #[serde(rename = "sale_time_begin")]
    pub sale_time_begin: String,
    #[serde(rename = "sale_type")]
    pub sale_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "realname_auth")]
    pub realname_auth: Option<String>,
    #[serde(rename = "suit_card_type")]
    pub suit_card_type: Option<String>,
    #[serde(rename = "pub_btn_plus_color")]
    pub pub_btn_plus_color: Option<String>,
    #[serde(rename = "pub_btn_shade_color_bottom")]
    pub pub_btn_shade_color_bottom: Option<String>,
    #[serde(rename = "pub_btn_shade_color_top")]
    pub pub_btn_shade_color_top: Option<String>,
    #[serde(rename = "rank_investor_name")]
    pub rank_investor_name: Option<String>,
    #[serde(rename = "sale_time_end")]
    pub sale_time_end: Option<String>,
    #[serde(rename = "related_words")]
    pub related_words: Option<String>,
    #[serde(rename = "realname_auth_age")]
    pub realname_auth_age: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentActivity {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "time_limit")]
    pub time_limit: bool,
    #[serde(rename = "time_left")]
    pub time_left: i64,
    pub tag: String,
    #[serde(rename = "price_bp_month")]
    pub price_bp_month: i64,
    #[serde(rename = "price_bp_forever")]
    pub price_bp_forever: i64,
}
