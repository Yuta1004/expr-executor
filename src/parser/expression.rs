use std::option::Option;

#[derive(PartialOrd, PartialEq)]
pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div,
    Num
}

pub struct Expression<'a> {
    kind: OperatorKind,
    val_a: Option<&'a Expression<'a>>,
    val_b: Option<&'a Expression<'a>>,
    num: i32
}

impl<'a> Expression<'a> {

    pub fn new(kind: OperatorKind, val_a: &'a Expression<'a>, val_b: &'a Expression<'a>) -> Expression<'a> {
        Expression{ kind, val_a: Some(val_a), val_b: Some(val_b), num: 0 }
    }

    pub fn new_num(val: i32) -> Expression<'a> {
        Expression{ kind: OperatorKind::Num, val_a: None, val_b: None, num: val }
    }

    pub fn calc(&self) -> i32 {
        // Num
        if self.kind == OperatorKind::Num {
            return self.num;
        }

        // Add ~ Div
        let a = self.val_a.unwrap().calc();
        let b = self.val_b.unwrap().calc();
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
    let expr_add = Expression::new(OperatorKind::Add, &num_1, &num_2);
    let expr_sub = Expression::new(OperatorKind::Sub, &num_0, &num_4);
    let expr_mul = Expression::new(OperatorKind::Mul, &expr_add, &expr_sub);
    let expr_div = Expression::new(OperatorKind::Div, &expr_mul, &num_6);
    assert_eq!(expr_div.calc(), -2);
}
