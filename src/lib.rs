#[macro_use]
extern crate serde_qs as qs;

use crate::login::{
    get_login_prepare_response, polling_login_info, read_user_info_from_file, user_info_params,
};
use anyhow::Result;
// 错误处理
use dialoguer::Confirm;
use reqwest;
// 网络请求
use std::io::Read; // 读取文件 // 用户交互

use login::check_login_status;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};


mod client_builder;
mod header;
pub mod login;
mod ticker;
mod client;

// UA string to pass to ClientBuilder.user_agent

pub async fn main_process () -> Result<()> {
    let login_status =  check_login_status().await;
    if let Ok(login_status) = login_status {
        // 登录成功
        println!("{:?}", login_status);
        let init_select = vec!["检查当前售卖装扮", "抢购装扮"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&init_select)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        match selection {
            Some(index) => {
                if index == 0 {
                    // 检查当前售卖装扮
                } else {
                    // 抢购装扮
                }
            }
            None => println!("没有选择，退出程序"),
        }
    } else {
        println!("登录系统出错，结束程序～");
    }
    Ok(())
}
