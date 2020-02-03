use expr_executor::parser::expression::{OperatorKind, Expression};

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
    let num_0 = Expression::new_num(0);
    let num_1 = Expression::new_num(1);
    let num_2 = Expression::new_num(2);
    let num_4 = Expression::new_num(4);
    let expr_add = Expression::new(OperatorKind::Add, &num_1, &num_2);
    let expr_sub = Expression::new(OperatorKind::Sub, &num_0, &num_4);
    let expr_mul = Expression::new(OperatorKind::Mul, &expr_add, &expr_sub);
    println!("(1 + 2) * (0 - 4) = {}", expr_mul.calc());
}
