use std::fmt;

pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div,
    Num
}

pub struct Expression {
    kind: OperatorKind,
    val_a: i32,
    val_b: i32
}

impl Expression {

    pub fn new(kind: OperatorKind, val_a: i32, val_b: i32) -> Expression {
        Expression{ kind, val_a, val_b }
    }

    pub fn new_num(val: i32) -> Expression {
        Expression{ kind: OperatorKind::Num, val_a: val, val_b: 0}
    }

    pub fn calc(&self) -> i32 {
        match &self.kind {
            OperatorKind::Add => self.val_a + self.val_b,
            OperatorKind::Sub => self.val_a - self.val_b,
            OperatorKind::Mul => self.val_a * self.val_b,
            OperatorKind::Div => self.val_b / self.val_b,
            OperatorKind::Num => self.val_a
        }
    }

}

impl fmt::Display for Expression {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_op = match &self.kind {
            OperatorKind::Add => "+",
            OperatorKind::Sub => "-",
            OperatorKind::Mul => "*",
            OperatorKind::Div => "/",
            OperatorKind::Num => ""
        };
        write!(f, "{} {} {}", str_op, self.val_a, self.val_b)
    }

}
