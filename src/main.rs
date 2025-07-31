use arboard::Clipboard;
use clap::Parser;
use rand::distr::{Distribution, Uniform};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 16)]
    length: usize,
    #[arg(short = 'n', long)]
    number: bool,
    #[arg(short = 's', long)]
    symbol: bool,
    #[arg(short = 'u', long)]
    uppercase: bool,
    #[arg(short = 'a', long)]
    all: bool,
    #[arg(short = 'c', long)]
    copy: bool,
}

fn main() {
    let mut args = Args::parse();

    if args.all {
        args.uppercase = true;
        args.number = true;
        args.symbol = true;
    }

    let mut charset = String::from("abcdefghijklmnopqrstuvwxyz");
    if args.uppercase {
        charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }

    if args.number {
        charset.push_str("0123456789");
    }

    if args.symbol {
        charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        eprintln!("エラー: 文字種が選択されていません。");
        std::process::exit(1);
    }

    let charset_chars: Vec<char> = charset.chars().collect();
    let between = Uniform::try_from(0..charset_chars.len()).unwrap();
    let mut rng = rand::rng();

    let password: String = (0..args.length)
        .map(|_| charset_chars[between.sample(&mut rng)])
        .collect();

    println!("{}", password);

    if args.copy {
        if let Ok(mut clipboard) = Clipboard::new() {
            if let Err(e) = clipboard.set_text(password.clone()) {
                eprintln!("クリップボードコピー失敗: {}", e);
            } else {
                println!("→ クリップボードにコピーされました。");
            }
        }
    }
}
