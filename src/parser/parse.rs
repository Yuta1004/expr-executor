use super::expression::Expression;

pub fn parse(expr: &str) -> Expression {
    let (_, out) = op_num(expr);
    out
}

fn op_num(expr: &str) -> (&str, Expression){
    let (num_s, expr) = find_num(skip_space(expr));
    if num_s == "" {
        panic!("\nCannot convert to num : {}
                        ^ here", expr);
    };
    (expr, Expression::new_num(num_s.parse::<i32>().unwrap()))
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

fn find_num(target: &str) -> (&str, &str) {
    for (idx, c) in target.chars().enumerate() {
        match c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {},
            _ => return (&target[..idx], &target[idx..])
        }
    };
    (target, "")
}

#[test]
fn test_parse() {
    assert_eq!(parse("1").calc(), 1);
    assert_eq!(parse("134").calc(), 134);
}

#[test]
fn test_skip_space() {
    assert_eq!(skip_space("test"), "test");
    assert_eq!(skip_space(" AB C D"), "AB C D");
    assert_eq!(skip_space("    GitHu b .com"), "GitHu b .com");
    assert_eq!(skip_space("....... test"), "....... test");
    assert_eq!(skip_space("  ..//+![2@]0"), "..//+![2@]0");
}

#[test]
fn test_find_num_from_top() {
    assert_eq!(find_num("19"), ("19", ""));
    assert_eq!(find_num(" 180"), ("", " 180"));
    assert_eq!(find_num("18 9"), ("18", " 9"));
    assert_eq!(find_num("1234567 89"), ("1234567", " 89"));
    assert_eq!(find_num("test1204"), ("", "test1204"));
    assert_eq!(find_num("1327++--1"), ("1327", "++--1"));
}