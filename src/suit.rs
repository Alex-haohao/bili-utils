use crate::bili_resp::suit_all::SuitAllResp;
use crate::bili_resp::suit_detail::SuitDetailResp;
use crate::client::parse_cookies;
use crate::user_info_params;
use crate::utils::get_bili_server_time;
use crate::utils::random_id;
use anyhow::Result;
use chrono::{Local, Utc};
use console::Term;
use crossbeam_channel::tick;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

lazy_static! {
    #[derive(Debug)]
static ref Buvid: String = random_id(37, true);
    #[derive(Debug)]
static ref device_id: String = random_id(38, true);
    #[derive(Debug)]
static ref fp_local: String = random_id(64, false);
    #[derive(Debug)]
static ref fp_remove: String = random_id(64, false);
    #[derive(Debug)]
static ref deviceFingerprint: String = random_id(32, true);
    #[derive(Debug)]
static ref BiliApp: String = "65500100".parse().unwrap();
    #[derive(Debug)]
static ref mobiApp: String = "iphone_b".parse().unwrap();
    #[derive(Debug)]
static ref buildId: String = "65500100".parse().unwrap();
    #[derive(Debug)]
static ref c_locale: String = "zh-Hans_CN".parse().unwrap();
    #[derive(Debug)]
static ref s_locale: String = "zh-Hans_CN".parse().unwrap();
    #[derive(Debug)]
static ref session_id: String = random_id(8, false);
static ref UA: String = format!("Mozilla/5.0 (iPhone; CPU iPhone OS 15_2 like Mac OS X) AppleWebKit/612.3.6.1.6 (KHTML, like Gecko) Mobile/21C52 BiliApp/{} os/ios model/iPad Pro 12.9-Inch 3G mobi_app/{} build/{} osVer/15.2 network/2 channel/AppStore Buvid/{} c_locale/{} s_locale/{} sessionID/{} disable_rcmd/0",*BiliApp,*mobiApp,*buildId,*Buvid,*c_locale,*s_locale,*session_id);
}

pub async fn checking_all_selling(cookies: &user_info_params) -> Result<SuitAllResp> {
    println!("{:?}", &*UA);
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .user_agent(&*UA)
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

pub async fn handle_buy_suit(cookies: &user_info_params) -> Result<()> {
    let test_suit = get_suit_detail(cookies, 33960).await?;
    let suit_id = test_suit.data.item.item_id;
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    let headers = construct_headers(cookies);
    // 1. create-order 创建订单
    let new_order = create_order(cookies, &headers, &client, 1, 1, suit_id).await?;
    // if new_order.code != 0 {
    //     println!("{}", new_order.message);
    //     return Ok(());
    // }

    // 2. confirm-order 确认订单
    // 每200ms轮询一次
    let ticker = tick(Duration::from_millis(200));
    loop {
        ticker.recv().unwrap();
        let confirmed_order =
            confirm_order(cookies, &headers, &client, &new_order.data.order_id).await?;
        if confirmed_order.code != 0 {
            println!("{}", confirmed_order.message);
            return Ok(());
        }
        if confirmed_order.data.expect("订单确认失败").state == "created" {
            println!("订单创建成功");
            break;
        }
    }
    Ok(())
}

// 创建订单
pub async fn create_order(
    cookies: &user_info_params,
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    add_month: u32,
    buy_num: u32,
    item_id: i64,
) -> Result<CreateOrderResp> {
    let mut post_data = HashMap::new();
    post_data.insert("add_month", add_month.to_string());
    post_data.insert("buy_num", buy_num.to_string());
    post_data.insert("csrf", cookies.bili_jct.clone());
    post_data.insert("hasBiliapp", true.to_string());
    post_data.insert("currency", "bp".to_string());
    post_data.insert("item_id", item_id.to_string());
    post_data.insert("platform", "ios".to_string());

    let res = client
        .post("https://api.bilibili.com/x/garb/trade/create")
        .headers(headers.clone())
        .form(&post_data)
        .send()
        .await?
        .json::<CreateOrderResp>()
        .await?;

    Ok(res)
}

// 确认订单 ConfirmOrder
pub async fn confirm_order(
    cookies: &user_info_params,
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    order_id: &String,
) -> Result<ConfirmOrderResp> {
    let mut post_data = HashMap::new();
    post_data.insert("csrf", cookies.bili_jct.clone());
    post_data.insert("order_id", "123".to_string());

    let res = client
        .post("https://api.bilibili.com/x/garb/trade/confirm")
        .headers(headers.clone())
        .form(&post_data)
        .send()
        .await?
        .json::<ConfirmOrderResp>()
        .await?;

    Ok(res)
}

//支付
pub async fn pay_pay(
    cookies: &user_info_params,
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    order_id: &String,
) -> Result<ConfirmOrderResp> {
    let mut post_data = HashMap::new();
    post_data.insert("csrf", cookies.bili_jct.clone());
    post_data.insert("order_id", "123".to_string());

    let res = client
        .post("https://pay.bilibili.com/payplatform/pay/pay")
        .headers(headers.clone())
        .form(&post_data)
        .send()
        .await?
        .json::<ConfirmOrderResp>()
        .await?;

    Ok(res)
}

/**
 * 处理预购的装扮
 */
pub async fn handle_pre_sale(
    cookies: &user_info_params,
    suit_detail: &SuitDetailResp,
) -> Result<()> {
    let suit_properties = suit_detail.data.item.properties.clone().unwrap();
    let sale_time_begin = suit_properties.sale_time_begin.parse::<i64>().unwrap();
    let sale_quantity = suit_properties.sale_quantity.parse::<i64>().unwrap();
    let sale_surplus = suit_detail.data.sale_surplus;
    //当前编号
    let next_number = sale_quantity - sale_surplus + 1;
    println!("{}", next_number);
    //计算倒计时
    let mut count_down = sale_time_begin - get_bili_server_time().await?;
    println!("{}", count_down);

    Ok(())
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

// 创建 header
use crate::bili_resp::confirm_order::ConfirmOrderResp;
use crate::bili_resp::create_order::CreateOrderResp;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE,
    REFERER, USER_AGENT,
};

fn construct_headers(cookies: &user_info_params) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(&*UA));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("zh-CN,zh-Hans;q=0.9"),
    );
    headers.insert(
        REFERER,
        HeaderValue::from_static("https://www.bilibili.com/h5/mall/suit/detail"),
    );
    headers.insert(
        "X-CSRF-TOKEN",
        cookies.bili_jct.clone().parse().expect("解析cookies失败"),
    );
    headers.insert(
        "Cookie",
        parse_cookies(cookies.clone())
            .parse()
            .expect("解析cookies失败"),
    );
    headers
}

// fn construct_pay_headers(cookies: &user_info_params) -> HeaderMap {
//     let mut headers = HeaderMap::new();
//     headers.insert(
//         ACCEPT_ENCODING,
//         HeaderValue::from_static("gzip, deflate, br"),
//     );
//     headers.insert(USER_AGENT, HeaderValue::from_static(UA));
//     headers.insert(
//         CONTENT_TYPE,
//         HeaderValue::from_static("application/x-www-form-urlencoded"),
//     );
//     headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
//     headers.insert(
//         ACCEPT_LANGUAGE,
//         HeaderValue::from_static("zh-CN,zh-Hans;q=0.9"),
//     );
//     headers.insert(
//         REFERER,
//         HeaderValue::from_static("https://www.bilibili.com/h5/mall/suit/detail"),
//     );
//     headers.insert(
//         "Cookie",
//         parse_cookies(cookies.clone())
//             .parse()
//             .expect("解析cookies失败"),
//     );
//     headers.insert(
//         "cLocale",
//         "zh_CN".parse().expect("解析cLocale失败"),
//     );
//     headers.insert(
//         "sLocale",
//         "zh_CN".parse().expect("解析sLocale失败"),
//     );
//     headers.insert(
//         "Buvid",
//         cookies..clone().parse().expect("解析Buvid失败"),
//     );
//     "Buvid": Buvid,
//     "Device-ID": device_id,
//     "fp_local": fp_local,
//     "fp_remote": fp_remove,
//     "session_id": session_id,
//     "deviceFingerprint": devicefingerprint,
//     "buildId": appVer,
//     "env": "prod",
//     "APP-KEY": "android",
//     "User-Agent": pay_User_Agent,
//     "bili-bridge-engine": "cronet",
//     headers
// }
