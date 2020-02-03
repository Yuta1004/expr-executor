use std::fmt;

pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div
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

    pub fn calc(&self) -> i32 {
        match &self.kind {
            OperatorKind::Add => self.val_a + self.val_b,
            OperatorKind::Sub => self.val_a - self.val_b,
            OperatorKind::Mul => self.val_a * self.val_b,
            OperatorKind::Div => self.val_b / self.val_b
        }
    }

}

impl fmt::Display for Expression {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_op = match &self.kind {
            OperatorKind::Add => "+",
            OperatorKind::Sub => "-",
            OperatorKind::Mul => "*",
            OperatorKind::Div => "/"
        };
        write!(f, "{} {} {}", str_op, self.val_a, self.val_b)
    }

}
