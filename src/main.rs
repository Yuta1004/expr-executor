use expr_executor::parser;

use std::env;

fn main() {
    // 引数
    let expr: &str;
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        2 => expr = &args[1],
        _ => panic!("Error!! Argument size must be 2!")
    }
    println!("Received Expr: {}", expr);

    parser::operator::test();
}
