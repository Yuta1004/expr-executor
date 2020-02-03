use std::fmt;
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

impl<'a> fmt::Display for Expression<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_op = match &self.kind {
            OperatorKind::Add => "+",
            OperatorKind::Sub => "-",
            OperatorKind::Mul => "*",
            OperatorKind::Div => "/",
            OperatorKind::Num => ""
        };
        write!(f, "{}", str_op)
    }

}
