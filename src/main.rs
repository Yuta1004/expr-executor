use expr_executor::parser::operator::{OperatorKind, Operator};

use std::env;

fn main() {
    // 引数
    let expr: &str;
    let args = env::args().collect::<Vec<_>>();
    match args.len() {
        2 => expr = &args[1],
        _ => panic!("Argument size must be 2")
    }
    println!("Received Expr: {}", expr);

    let add_op = Operator::new(OperatorKind::Add, 1, 2);
    println!("{}", add_op.calc());
}
