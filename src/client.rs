use crate::login::UA;
use crate::user_info_params;
use std::collections::HashMap;

pub fn parse_cookies(cookies: &user_info_params) -> String {
    let mut cookies_map: HashMap<String, String> = HashMap::new();
    cookies_map.insert("SESSDATA".into(), cookies.SESSDATA.clone());
    cookies_map.insert("DedeUserID".into(), cookies.DedeUserID.to_string());
    cookies_map.insert(
        "DedeUserID__ckMd5".into(),
        cookies.DedeUserID__ckMd5.clone(),
    );
    cookies_map.insert("Expires".into(), cookies.Expires.to_string());
    cookies_map.insert("bili_jct".into(), cookies.bili_jct.clone());

    let cookies_value: String = cookies_map
        .iter()
        // TODO: check if extra escaping is needed
        .map(|(k, v)| format!("{}={}", k, v).replace(";", "%3B"))
        .collect::<Vec<_>>()
        .join(";");

    cookies_value
}
