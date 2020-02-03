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
    println!("Expr: {}", expr);

    // (1 + 2) * (0 - 4)
    let add_op = Operator::new(OperatorKind::Add, 1, 2);
    let sub_op = Operator::new(OperatorKind::Sub, 0, 4);
    let mul_op = Operator::new(OperatorKind::Mul, add_op.calc(), sub_op.calc());
    println!("(1 + 2) * (0 + 4) = {}", mul_op.calc());
}
