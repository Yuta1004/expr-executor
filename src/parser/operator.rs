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

    pub fn calc(&self) -> i32 {
        match &self.kind {
            OperatorKind::Add => self.val_a + self.val_b,
            OperatorKind::Sub => self.val_a - self.val_b,
            OperatorKind::Mul => self.val_a * self.val_b,
            OperatorKind::Div => self.val_b / self.val_b
        }
    }

}
