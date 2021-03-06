use crate::bili_resp::suit_all::SuitAllResp;
use crate::bili_resp::suit_detail::SuitDetailResp;
use crate::client::parse_cookies;
use crate::user_info_params;
use crate::utils::random_id;
use crate::utils::{get_bili_server_time, get_current_local_time};
use anyhow::Result;
use chrono::{Local, Utc};
use console::Term;
use crossbeam_channel::tick;
use dialoguer::Confirm;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use fancy_regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

lazy_static! {
    #[derive(Debug)]
static ref buvid: String = "4069f7a0af9623e13a283c456eaf403".to_string();
      #[derive(Debug)]
static ref accessKey: String = random_id(32, false);
    #[derive(Debug)]
static ref device_id: String = random_id(38, true);
    #[derive(Debug)]
static ref fp_local: String = random_id(64, false);
    #[derive(Debug)]
static ref fp_remove: String = random_id(64, false);
    #[derive(Debug)]
static ref deviceFingerprint: String = "a009263b4e8fd4e82140f22055d53ac9".to_string();
    #[derive(Debug)]
static ref BiliApp: String = "6560300".parse().unwrap();
    #[derive(Debug)]
static ref mobiapp: String = "iphone".parse().unwrap();
    #[derive(Debug)]
static ref buildId: String = "65800100".parse().unwrap();
    #[derive(Debug)]
static ref c_locale: String = "zh-Hans_CN".parse().unwrap();
    #[derive(Debug)]
static ref s_locale: String = "zh-Hans_CN".parse().unwrap();
    #[derive(Debug)]
static ref session_id: String = random_id(8, false);
static ref UA: String = format!("Mozilla/5.0 (Linux; Android 10; HLK-AL10 Build/HONORHLK-AL10; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/76.0.3809.89 Mobile Safari/537.36 T7/12.6 SP-engine/2.26.0 baiduboxapp/12.6.0.10 (Baidu; P1 10) NABar/1.0 BiliApp/{} os/ios model/iPad Pro 12.9-Inch 3G mobi_app/{} build/{} osVer/15.2 network/2 channel/AppStore buvid/{} c_locale/{} s_locale/{} sessionID/{} disable_rcmd/0",*BiliApp,*mobiapp,*buildId,*buvid,*c_locale,*s_locale,*session_id);}

pub async fn checking_all_selling(cookies: &user_info_params) -> Result<SuitAllResp> {
    let client = reqwest::ClientBuilder::new()
        .cookie_store(true)
        .brotli(true)
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

    // ?????????????????????
    for category in &resp.data.category {
        category_vec.push(category.name.clone());
    }
    category_vec.push("???????????????".to_string());

    loop {
        //???????????????????????????
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&category_vec)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(category_index) => {
                // ????????????????????????????????????
                if category_index == category_vec.len() - 1 {
                    break;
                }

                let mut suit_vec: Vec<String> = Vec::new();

                // ???????????????suit
                for suit in &resp.data.category[category_index].suits {
                    suit_vec.push(suit.name.clone());
                }
                suit_vec.push("???????????????".to_string());

                loop {
                    // ?????????????????????suit
                    let suit_selection = Select::with_theme(&ColorfulTheme::default())
                        .items(&suit_vec)
                        .default(0)
                        .interact_on_opt(&Term::stderr())?;

                    // handle ???????????????
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
                        None => println!("??????????????????"),
                    }
                }
            }
            None => println!("??????????????????"),
        }
    }
    Ok(resp)
}

pub async fn buy_suit(cookies: &user_info_params) -> Result<()> {
    // ????????? ??????????????????????????????id
    //???????????????????????????bilibili????????????????????????
    let server_time = get_bili_server_time().await?;
    let local_time = Utc::now().timestamp();
    if server_time == local_time {
        println!("???????????????bilibili?????????????????????");
    } else {
    }

    //????????????????????????id
    let suit_id: String = Input::new()
        .with_prompt("???????????????????????????id")
        .with_initial_text("")
        .default("No".into())
        .interact_text()?;

    let suit_id = suit_id.parse::<i32>();
    if suit_id.is_err() {
        println!("???????????????id?????????");
        return Ok(());
    }
    let suit_id = suit_id.expect("???????????????id?????????");
    let res = get_suit_detail(cookies, suit_id).await;
    if let Err(e) = res {
        println!("??????id????????????");
        print!("{}", e);
        return Ok(());
    }
    let res = res.unwrap();
    if (res.data.sale_surplus <= 0) {
        println!("???????????????????????????");
        return Ok(());
    }

    // ??????????????????
    if res.data.item.properties.is_some() {
        let suit_properties = res.data.item.properties.clone().unwrap();
        // ???????????????
        if server_time > suit_properties.sale_time_begin.parse::<i64>().unwrap() {
            println!("???????????????");
        } else {
            //???????????????
            println!("???????????????");
            handle_pre_sale(cookies, &res).await?;
        }
    }

    Ok(())
}

pub async fn handle_buy_suit(
    cookies: &user_info_params,
    suit_id: i64,
    add_month: u32,
    buy_num: u32,
) -> Result<()> {
    let client = reqwest::ClientBuilder::new().cookie_store(true).build()?;
    let headers = construct_headers(cookies, &suit_id);
    let pay_header = construct_pay_headers(cookies);

    // 1. create-order ????????????
    let new_order = create_order(cookies, &headers, &client, add_month, buy_num, suit_id).await?;
    if new_order.code != 0 {
        println!("{}", new_order.message);
        return Ok(());
    }

    // 2. confirm-order ????????????
    // ???200ms????????????
    let ticker = tick(Duration::from_millis(200));
    let mut confirmed_order = Default::default();
    loop {
        ticker.recv().unwrap();
        confirmed_order =
            confirm_order(cookies, &headers, &client, &new_order.data.order_id).await?;
        if confirmed_order.code != 0 {
            return Ok(());
        }
        if confirmed_order.data.clone().expect("??????????????????").state == "created" {
            println!("??????????????????");
            break;
        }
    }

    //3. ????????????
    let (pay_order_res, pay_zone) = pay_pay(&pay_header, &client, confirmed_order).await?;
    if pay_order_res.errno == 0 {
        println!("??????????????????");
    } else {
        println!("{:?}", pay_order_res);
        println!("{}", pay_order_res.msg);
        return Ok(());
    }

    //4. ????????????
    let pay_bp_res = pay_bp(&pay_header, &client, pay_order_res).await;
    if let Err(e) = pay_bp_res {
        println!("{}", e);
        return Ok(());
    }
    let pay_bp_res = pay_bp_res.expect("????????????");
    if pay_bp_res.errno == 0 {
        let success = pay_bp_res.success;
        if success {
            println!("????????????");
        } else {
            println!("????????????");
        }
    } else {
        println!("{:?}", pay_bp_res);
        println!("{}", pay_bp_res.msg);
        return Ok(());
    }
    Ok(())
}

// ????????????
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

// ???????????? ConfirmOrder
pub async fn confirm_order(
    cookies: &user_info_params,
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    order_id: &String,
) -> Result<ConfirmOrderResp> {
    let mut post_data = HashMap::new();
    post_data.insert("csrf", cookies.bili_jct.clone());
    post_data.insert("order_id", order_id.to_string());

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

use serde_json::{Map, Number};

//??????????????????
pub async fn pay_pay(
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    orderDetail: ConfirmOrderResp,
) -> Result<(PayOrderResp, String)> {
    let mut post_data = Map::new();
    let pay_data = orderDetail.data.expect("??????paydata??????");
    let pay_data_json: PayDataResp = serde_json::from_str(pay_data.pay_data.as_str())?;

    post_data.insert(
        "originalAmount".to_string(),
        Value::String(pay_data_json.original_amount.to_string()),
    );
    post_data.insert(
        "orderCreateTime".to_string(),
        Value::String(pay_data_json.order_create_time.to_string()),
    );

    post_data.insert(
        "showTitle".to_string(),
        Value::String(pay_data_json.show_title.to_string()),
    );
    post_data.insert(
        "deviceType".to_string(),
        Value::Number(Number::from(pay_data_json.device_type.parse::<i64>()?)),
    );
    post_data.insert(
        "customerId".to_string(),
        Value::Number(pay_data_json.customer_id.into()),
    );
    post_data.insert(
        "orderExpire".to_string(),
        Value::String(pay_data_json.order_expire.to_string()),
    );
    post_data.insert(
        "productId".to_string(),
        Value::String(pay_data_json.product_id.to_string()),
    );
    post_data.insert(
        "version".to_string(),
        Value::String(pay_data_json.version.to_string()),
    );
    post_data.insert(
        "payAmount".to_string(),
        Value::String(pay_data_json.pay_amount.to_string()),
    );
    post_data.insert(
        "signType".to_string(),
        Value::String(pay_data_json.sign_type.to_string()),
    );
    post_data.insert(
        "sign".to_string(),
        Value::String(pay_data_json.sign.to_string()),
    );
    post_data.insert(
        "uid".to_string(),
        Value::String(pay_data_json.uid.to_string()),
    );
    post_data.insert(
        "timestamp".to_string(),
        Value::String(pay_data_json.timestamp.to_string()),
    );
    post_data.insert(
        "serviceType".to_string(),
        Value::String(pay_data_json.service_type.to_string()),
    );
    post_data.insert(
        "traceId".to_string(),
        Value::String(pay_data_json.trace_id.to_string()),
    );
    post_data.insert(
        "payChannelId".to_string(),
        Value::Number(Number::from(99 as i64)),
    );
    post_data.insert(
        "accessKey".to_string(),
        Value::String(accessKey.to_string()),
    );
    post_data.insert(
        "appVersion".to_string(),
        Value::String("6.58.0".to_string()),
    );
    post_data.insert("network".to_string(), Value::String("WIFI".to_string()));
    post_data.insert("device".to_string(), Value::String("iOS".to_string()));
    post_data.insert("sdkVersion".to_string(), Value::String("1.4.8".to_string()));
    post_data.insert("payChannel".to_string(), Value::String("bp".to_string()));
    post_data.insert(
        "appName".to_string(),
        Value::String("tv.danmaku.bilianime".to_string()),
    );
    post_data.insert(
        "notifyUrl".to_string(),
        Value::String(pay_data_json.notify_url.to_string()),
    );
    post_data.insert(
        "orderId".to_string(),
        Value::String(pay_data_json.order_id.to_string()),
    );

    let res = client
        .post("https://pay.bilibili.com/payplatform/pay/pay")
        .headers(headers.clone())
        .json(&post_data)
        .send()
        .await?;

    let pay_zone = res
        .headers()
        .get("Set-Cookie")
        .expect("??????payzone??????")
        .to_str()
        .expect("??????payzone??????");

    let re = Regex::new(r"(?!payzone=)\w+(?=;)").expect("?????????????????????");
    let pay_zone = re
        .find(pay_zone)
        .expect("Result payzone??????")
        .expect("option payzone??????")
        .as_str()
        .to_string();

    let res = res.json::<PayOrderResp>().await?;

    Ok((res, pay_zone))
}

//??????????????????
pub async fn pay_bp(
    headers: &reqwest::header::HeaderMap,
    client: &Client,
    pay_pay_resp: PayOrderResp,
) -> Result<PayBpResp> {
    let mut post_data = Map::new();
    let pay_data = pay_pay_resp.data.expect("??????pay_channel??????");
    let pay_channel_json: PayChannelParam =
        serde_json::from_str(pay_data.pay_channel_param.as_str())?;

    post_data.insert(
        "orderId".to_string(),
        Value::String(pay_channel_json.order_id.to_string()),
    );
    post_data.insert(
        "productId".to_string(),
        Value::String(pay_channel_json.product_id.to_string()),
    );
    post_data.insert(
        "feeType".to_string(),
        Value::String(pay_channel_json.fee_type.to_string()),
    );
    post_data.insert(
        "customerName".to_string(),
        Value::String(pay_channel_json.customer_name.to_string()),
    );
    post_data.insert(
        "noBpCoupon".to_string(),
        Value::String(pay_channel_json.no_bp_coupon.to_string()),
    );
    post_data.insert(
        "mid".to_string(),
        Value::String(pay_channel_json.mid.to_string()),
    );
    post_data.insert(
        "timestamp".to_string(),
        Value::String(pay_channel_json.timestamp.to_string()),
    );
    post_data.insert(
        "customerId".to_string(),
        Value::String(pay_channel_json.customer_id.to_string()),
    );
    post_data.insert(
        "txId".to_string(),
        Value::String(pay_channel_json.tx_id.to_string()),
    );
    post_data.insert(
        "remark".to_string(),
        Value::String(pay_channel_json.remark.to_string()),
    );
    post_data.insert(
        "platformType".to_string(),
        Value::String(pay_channel_json.platform_type.to_string()),
    );
    post_data.insert(
        "orderCreateTime".to_string(),
        Value::String(pay_channel_json.order_create_time.to_string()),
    );
    post_data.insert(
        "payAmout".to_string(),
        Value::String(pay_channel_json.pay_amout.to_string()),
    );
    post_data.insert(
        "sign".to_string(),
        Value::String(pay_channel_json.sign.to_string()),
    );

    let res = client
        .post("https://pay.bilibili.com/paywallet/pay/payBp")
        .headers(headers.clone())
        .json(&post_data)
        .send()
        .await?
        .json::<PayBpResp>()
        .await?;

    Ok(res)
}

/**
 * ?????????????????????
 */
pub async fn handle_pre_sale(
    cookies: &user_info_params,
    suit_detail: &SuitDetailResp,
) -> Result<()> {
    let suit_properties = suit_detail.data.item.properties.clone().unwrap();
    let sale_time_begin = suit_properties.sale_time_begin.parse::<i64>().unwrap();
    let sale_quantity = suit_properties.sale_quantity.parse::<i64>().unwrap();
    let sale_surplus = suit_detail.data.sale_surplus;
    //????????????
    let next_number = sale_quantity - sale_surplus + 1;
    println!("{}", next_number);
    //???????????????
    let mut count_down = sale_time_begin - get_current_local_time();

    // ???1???????????????????????????
    let ticker = tick(Duration::from_millis(1000));
    loop {
        ticker.recv().unwrap();
        println!("?????????:{}", count_down);
        count_down = sale_time_begin - get_current_local_time();
        if count_down <= 10 {
            break;
        }
    }

    println!("??????????????????");
    // ???????????????????????????????????????????????????
    loop {
        count_down = sale_time_begin - get_current_local_time();
        // ????????????????????????2???3????????? ????????????????????????bili server time ????????????
        if count_down <= 3 {
            break;
        }
    }

    Ok(())
}

// ??????????????????
pub async fn query_pay(
    cookies: &user_info_params,
    client: &Client,
    order_id: &String,
) -> Result<PayQueryResp> {
    let res = client
        .get("https://api.bilibili.com/x/garb/trade/query")
        .header("cookie", parse_cookies(cookies))
        .query(&[("order_id", order_id), ("csrf", &cookies.bili_jct)])
        .send()
        .await?
        .json::<PayQueryResp>()
        .await?;

    Ok(res)
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

// ?????? header
use crate::bili_resp::confirm_order::ConfirmOrderResp;
use crate::bili_resp::create_order::CreateOrderResp;
use crate::bili_resp::pay_bp::PayBpResp;
use crate::bili_resp::pay_channel_param::PayChannelParam;
use crate::bili_resp::pay_data::PayDataResp;
use crate::bili_resp::pay_order::PayOrderResp;
use crate::bili_resp::pay_query::PayQueryResp;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE,
    REFERER, USER_AGENT,
};

fn construct_headers(cookies: &user_info_params, suit_id: &i64) -> HeaderMap {
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
    let referString = format!("https://www.bilibili.com/h5/mall/suit/detail?{}", suit_id);
    headers.insert(REFERER, referString.as_str().parse().unwrap());
    headers.insert(
        "X-CSRF-TOKEN",
        cookies
            .bili_jct
            .clone()
            .parse()
            .expect("??????referString??????"),
    );
    headers.insert(
        "Cookie",
        parse_cookies(cookies.clone())
            .parse()
            .expect("??????cookies??????"),
    );
    headers
}

pub fn construct_pay_headers(cookies: &user_info_params) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(UA.as_str()));
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
    headers.insert("buvid", buvid.parse().expect("??????Buvid??????"));
    headers.insert(
        "session_id",
        session_id.parse().expect("??????session_id??????"),
    );
    headers.insert(
        "deviceFingerprint",
        deviceFingerprint
            .parse()
            .expect("??????deviceFingerprint??????"),
    );
    headers.insert("buildId", buildId.parse().expect("??????buildId??????"));
    headers.insert("env", "prod".parse().expect("??????env??????"));
    headers.insert("User-Agent", UA.parse().expect("??????UA??????"));
    headers.insert("mobiapp", mobiapp.parse().expect("??????mobiapp??????"));
    headers.insert(
        "Content-Type",
        "application/json".parse().expect("??????bridge??????"),
    );
    headers
}
