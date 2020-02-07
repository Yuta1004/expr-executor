use expr_executor::parser::parse::parse;

use std::env;
use std::io;

fn main() {
    // 引数
    let expr: &str;
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        2 => expr = &args[1],
        _ => panic!("Usage: cargo run <EXPR>")
    }
    println!("Expr: {}", expr);

    // 計算
    println!("Answer: {}", parse(expr).calc());
}

/// 標準入力から1行読み込んで返す
///
/// # Params
///
/// # Return
/// - String : 入力された文字列
fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim().parse().ok().unwrap()
}
