use clap::Parser;

/// 간단한 예제 프로그램
#[derive(Parser)]
#[command(name = "hello-cli", version, about = "Rust CLI 예제")]
struct Cli {
    /// 인사할 대상
    #[arg(default_value = "World")]
    name: String,

    /// 대문자로 인사할지 여부
    #[arg(short, long, action)]
    shout: bool,
}

fn main() {
    let args = Cli::parse();

    let mut greeting = format!("Hello, {}!", args.name);
    if args.shout {
        greeting = greeting.to_uppercase();
    }
    println!("{greeting}");
}
