use crate::login::UA;
use crate::user_info_params;
use anyhow::Result;
use reqwest::Client;
use std::collections::HashMap;

pub fn build_client(cookies: user_info_params) -> Result<Client> {
    let mut cookiesMap: HashMap<String, String> = HashMap::new();
    cookiesMap.insert("SESSDATA".into(), cookies.SESSDATA.into());
    cookiesMap.insert("DedeUserID".into(), cookies.DedeUserID.to_string());
    cookiesMap.insert("DedeUserID__ckMd5".into(), cookies.DedeUserID__ckMd5.into());
    cookiesMap.insert("Expires".into(), cookies.Expires.to_string());
    cookiesMap.insert("bili_jct".into(), cookies.bili_jct.into());

    let cookies_value: String = cookiesMap
        .iter()
        // TODO: check if extra escaping is needed
        .map(|(k, v)| format!("{}={}", k, v).replace(";", "%3B"))
        .collect::<Vec<_>>()
        .join(";");

    let client = reqwest::ClientBuilder::new()
        .user_agent(UA)
        .cookie_store(true)
        .build()?;

    Ok(client)
}
