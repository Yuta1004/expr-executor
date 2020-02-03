pub enum OperatorKind {
    ADD,
    SUB,
    MUL,
    DIV
}

pub struct Operator {
    kind: OperatorKind,
    val_a: i32,
    val_b: i32
}

impl Operator {

    pub fn calc(&self) -> i32 {
        match &self.kind {
            ADD => self.val_a + self.val_b,
            SUB => self.val_a - self.val_b,
            MUL => self.val_a * self.val_b,
            DIV => self.val_b / self.val_b
        }
    }

}
