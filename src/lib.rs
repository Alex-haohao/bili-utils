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

mod client_builder;
mod header;
pub mod login;
mod ticker;

// UA string to pass to ClientBuilder.user_agent
