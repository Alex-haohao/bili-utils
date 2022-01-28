use clap::Parser;
use bili_suit::test;

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

fn main() {
    let cli = Args::parse();

    /*
    当用户输入check时，执行检查当前售卖的物品列表。
    */
    if let true = cli.check {
        test();
        return;
    }

    if let true = cli.did {
        println!("Value for name: {}", 2);
        return;
    }
}
