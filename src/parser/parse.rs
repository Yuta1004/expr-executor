use super::expression::Expression;

pub fn parse(expr: &str) -> Expression {
    Expression::new_num(0)
}


fn skip_space(target: &str) -> &str {
    for (idx, c) in target.chars().enumerate() {
        match c {
            ' ' => {},
            _ => return &target[idx..]
        }
    };
    ""
}

#[test]
fn test_skip_space() {
    assert_eq!(skip_space("test"), "test");
    assert_eq!(skip_space(" AB C D"), "AB C D");
    assert_eq!(skip_space("    GitHu b .com"), "GitHu b .com");
    assert_eq!(skip_space("....... test"), "....... test");
    assert_eq!(skip_space("  ..//+![2@]0"), "..//+![2@]0");
}
