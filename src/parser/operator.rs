pub fn test() {
    println!("test called!");
}

enum OperatorKind {
    ADD,
    SUB,
    MUL,
    DIV
}

struct Operator {
    kind: OperatorKind,
    val_a: i32,
    val_b: i32
}

trait Calc {
    fn calculate(&self) -> i32;
}

