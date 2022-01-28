use bili_suit::test;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "bili-suit")]
#[clap(author = "Alexhaohao")]
#[clap(version = "1.0")]
#[clap(about = "get suit", long_about = None)]
struct Args {
    /// check curring sealing
    #[clap(short, long)]
    check: bool,

    #[clap(short, long)]
    did: bool,
}

// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    let cli = Args::parse();

    /*
    当用户输入check时，执行检查当前售卖的物品列表。
    */
    if let true = cli.check {
        let a = test().await;
        println!("{:?}", a);
        return;
    }

    if let true = cli.did {
        println!("Value for name: {}", 2);
        return;
    }
}
