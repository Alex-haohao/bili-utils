use bili_suit::check_login_status;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Select};

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() -> std::io::Result<()> {
    check_login_status().await;

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

    Ok(())
}
