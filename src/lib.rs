#[macro_use]
extern crate serde_qs as qs;

#[macro_use]
extern crate lazy_static;

use crate::login::{
    get_login_prepare_response, polling_login_info, read_user_info_from_file, test_login_status,
    user_info_params,
};
use crate::suit::{buy_suit, checking_all_selling, handle_buy_suit};
use anyhow::Result;
// 错误处理
use dialoguer::Confirm;
use reqwest;
// 网络请求
use std::io::Read; // 读取文件 // 用户交互

use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use login::check_login_status;

mod bili_resp;
mod client;
mod client_builder;
mod header;
pub mod login;
mod suit;
mod ticker;
mod utils;

/**
 * 主流程
 */
pub async fn main_process() -> Result<()> {
    let login_status = check_login_status().await;
    if let Ok(cookies) = login_status {
        // 获取cookies成功，测试是否能够登录
        let cookies = test_login_status(cookies).await?;

        let mut init_select = vec!["检查当前售卖装扮", "抢购装扮", "直接购买-测试"];
        init_select.push("退出程序");

        loop {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&init_select)
                .default(0)
                .interact_on_opt(&Term::stderr())?;

            match selection {
                Some(index) => {
                    // 如果选择最后一个选项，退出程序
                    if index == init_select.len() - 1 {
                        break;
                    }
                    if index == 0 {
                        // 检查当前售卖装扮
                        handle_check_all_suit(&cookies).await?;
                    } else if index == 1 {
                        // 抢购装扮
                        buy_suit(&cookies).await?;
                    } else {
                        // 直接购买-测试
                        handle_buy_suit(&cookies).await?;
                    }
                }
                None => println!("没有选择，退出程序"),
            }
        }
    } else {
        println!("登录系统出错，结束程序～");
    }

    Ok(())
}

pub async fn handle_check_all_suit(cookies: &user_info_params) -> Result<()> {
    checking_all_selling(cookies).await?;
    Ok(())
}
