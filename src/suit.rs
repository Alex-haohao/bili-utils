use crate::bili_resp::suit_all::SuitAllResp;
use crate::client::parse_cookies;
use crate::login::UA;
use crate::user_info_params;
use anyhow::Result;
use console::Term;
use crossbeam_channel::tick;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Select};
use qrcode::render::unicode;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{fs, process, thread};
use url::Url;

pub async fn checking_all_selling(cookies: &user_info_params) -> Result<SuitAllResp> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent(UA)
        .build()?;
    let cookies_header = parse_cookies(cookies);
    let url = "https://api.bilibili.com/x/garb/mall/suit/all";
    let resp = client
        .get(url)
        .header("cookie", cookies_header)
        .send()
        .await?
        .json::<SuitAllResp>()
        .await?;

    let mut category_vec: Vec<String> = Vec::new();

    // 收集所有的分类
    for category in &resp.data.category {
        category_vec.push(category.name.clone());
    }
    category_vec.push("返回上一步".to_string());

    loop {
        //用户选择想要的分类
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&category_vec)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(category_index) => {
                // 如果用户选择了返回上一步
                if category_index == category_vec.len() - 1 {
                    break;
                }

                let mut suit_vec: Vec<String> = Vec::new();

                // 收集所有的suit
                for suit in &resp.data.category[category_index].suits {
                    suit_vec.push(suit.name.clone());
                }
                suit_vec.push("返回上一步".to_string());

                loop {
                    // 选择想要查看的suit
                    let suit_selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&suit_vec)
                        .default(0)
                        .interact_on_opt(&Term::stderr())?;

                    // handle 用户的选择
                    match suit_selection {
                        Some(suit_index) => {
                            if suit_index == suit_vec.len() - 1 {
                                break;
                            }
                            println!("------------------------------------");
                            println!(
                                "name: {}",
                                resp.data.category[category_index].suits[suit_index].name
                            );
                            println!(
                                "item_id: {}",
                                resp.data.category[category_index].suits[suit_index].item_id
                            );
                            println!(
                                "state: {}",
                                resp.data.category[category_index].suits[suit_index].state
                            );
                            println!("------------------------------------");
                            break;
                        }
                        None => println!("无法查看装扮"),
                    }
                }
            }
            None => println!("无法选择分类"),
        }
    }

    Ok(resp)
}
