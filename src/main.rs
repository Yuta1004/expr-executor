use expr_executor::parser::parse::parse;

use std::env;

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
