pub enum OperatorKind {
    Add,
    Sub,
    Mul,
    Div
}

pub struct Operator {
    kind: OperatorKind,
    val_a: i32,
    val_b: i32
}

impl Operator {

    pub fn new(kind: OperatorKind, val_a: i32, val_b: i32) -> Operator {
        Operator{ kind, val_a, val_b }
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
