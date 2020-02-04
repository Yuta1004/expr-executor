use std::option::Option;

#[derive(PartialOrd, PartialEq)]
pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div,
    Num
}

pub struct Expression {
    kind: OperatorKind,
    val_a: Box<Option<Expression>>,
    val_b: Box<Option<Expression>>,
    num: i32
}

impl Expression {

    pub fn new(kind: OperatorKind, val_a: Expression, val_b: Expression) -> Expression {
        Expression{ kind, val_a: Box::new(Some(val_a)), val_b: Box::new(Some(val_b)), num: 0 }
    }

    pub fn new_num(val: i32) -> Expression {
        Expression{ kind: OperatorKind::Num, val_a: Box::new(None), val_b: Box::new(None), num: val }
    }

    pub fn calc(&self) -> i32 {
        // Num
        if self.kind == OperatorKind::Num {
            return self.num;
        }

        // Add ~ Div
        let a = match &*self.val_a {
            Some(a) => a.calc(),
            None => panic!("failed unwrap pval_a")
        };
        let b = match &*self.val_b {
            Some(b) => b.calc(),
            None => panic!("failed unwrap val_b")
        };
        match &self.kind {
            OperatorKind::Add => a + b,
            OperatorKind::Sub => a - b,
            OperatorKind::Mul => a * b,
            OperatorKind::Div => a / b,
            _ => 0
        }
    }

}

#[test]
fn test_expression() {
    // (1+2) * (0-4) / 6
    let num_0 = Expression::new_num(0);
    let num_1 = Expression::new_num(1);
    let num_2 = Expression::new_num(2);
    let num_4 = Expression::new_num(4);
    let num_6 = Expression::new_num(6);
    let expr_add = Expression::new(OperatorKind::Add, num_1, num_2);
    let expr_sub = Expression::new(OperatorKind::Sub, num_0, num_4);
    let expr_mul = Expression::new(OperatorKind::Mul, expr_add, expr_sub);
    let expr_div = Expression::new(OperatorKind::Div, expr_mul, num_6);
    assert_eq!(expr_div.calc(), -2);
}
