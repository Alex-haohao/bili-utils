use crate::bili_resp::suit_all::SuitAllResp;
use crate::bili_resp::suit_detail::SuitDetailResp;
use crate::client::parse_cookies;
use crate::login::UA;
use crate::user_info_params;
use crate::utils::get_bili_server_time;
use anyhow::Result;
use chrono::{Local, Utc};
use console::Term;
use crossbeam_channel::tick;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

pub async fn buy_suit(cookies: &user_info_params) -> Result<()> {
    // 第一步 用户输入要购买的装扮id

    //测试一下本地时间和bilibili服务器的时间差距
    let server_time = get_bili_server_time().await?;
    let local_time = Utc::now().timestamp();
    if server_time == local_time {
        println!("本地时间和bilibili服务器时间一致");
    } else {
    }
    println!("{}", server_time);
    println!("{}", local_time);

    //输入要购买的装扮id
    let suit_id: String = Input::new()
        .with_prompt("请输入要购买的装扮id")
        .with_initial_text("")
        .default("No".into())
        .interact_text()?;

    let suit_id = suit_id.parse::<i32>();
    if suit_id.is_err() {
        println!("输入的装扮id不合法");
        return Ok(());
    }
    let suit_id = suit_id.expect("输入的装扮id不合法");
    let res = get_suit_detail(cookies, suit_id).await;
    if let Err(e) = res {
        println!("装扮id检索出错");
        print!("{}", e);
        return Ok(());
    }
    let res = res.unwrap();
    if (res.data.sale_surplus <= 0) {
        println!("装扮未查询或已售罄");
        return Ok(());
    }

    // 预购装扮购买
    if res.data.item.properties.is_some() {
        let suit_properties = res.data.item.properties.clone().unwrap();
        // 装扮已开卖
        if server_time > suit_properties.sale_time_begin.parse::<i64>().unwrap() {
            println!("装扮已开卖");
        } else {
            //装扮未开卖
            println!("装扮未开卖");
            handle_pre_sale(cookies, &res).await;
        }
    }

    Ok(())
}

// pub async fn count_down {
//     let (tx, rx) = tick(1);
//     let mut count = 0;
//     loop {
//         let _ = rx.recv().await;
//         count += 1;
//         println!("{}", count);
//     }
// }

/**
 * 处理预购的装扮
 */
pub async fn handle_pre_sale(cookies: &user_info_params, suit_detail: &SuitDetailResp) {
    let suit_properties = suit_detail.data.item.properties.clone().unwrap();
    let sale_time_begin = suit_properties.sale_time_begin.parse::<i64>().unwrap();
    let sale_quantity = suit_properties.sale_quantity.parse::<i64>().unwrap();
    let sale_surplus = suit_detail.data.sale_surplus;
    //当前编号
    let next_number = sale_quantity - sale_surplus + 1;
    println!("{}", next_number);
}

pub async fn get_suit_detail(cookies: &user_info_params, suit_id: i32) -> Result<SuitDetailResp> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.bilibili.com/x/garb/mall/item/suit/v2")
        .query(&[("part", "suit"), ("item_id", &suit_id.to_string())])
        .header("cookie", parse_cookies(cookies))
        .send()
        .await?
        .json::<SuitDetailResp>()
        .await?;
    Ok(res)
}
