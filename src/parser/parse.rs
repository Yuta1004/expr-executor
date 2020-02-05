use super::expression::{Expression, OperatorKind};

/// 式をパースしてExpressionで表現した形にする
///
/// # Params
/// - expr (&str) : 式
///
/// # Return
/// - Expression : Expressionで表現された式
///
/// # Examples
/// ```
/// use expr_executor::parser::parse::parse;
/// parse("1").calc(); // -> 1
/// ```
pub fn parse(expr: &str) -> Expression {
    let (_, out) = op_add_sub(expr);
    out
}

/// 足し算/引き算をパースする
/// parseによって呼ばれる
///
/// # Params
/// - expr (&str) : 式
///
/// # Return
/// - &str : 評価に使用した部分を除いた後の式
/// - Expression : Expressionで表現された式
fn op_add_sub(expr: &str) -> (&str, Expression) {
    let (mut expr, mut node) = op_mul_div(expr);
    loop {
        expr = skip_space(expr);
        let op = match expr.chars().nth(0) {
            Some('+') => OperatorKind::Add,
            Some('-') => OperatorKind::Sub,
            _ => break
        };
        let (tmp_expr, right) = op_mul_div(&expr[1..]);
        expr = tmp_expr;
        node = Expression::new(op, node, right);
    }
    (expr, node)
}

/// 掛け算/割り算をパースする
/// parseによって呼ばれる
///
/// # Params
/// - expr (&str) : 式
///
/// # Return
/// - &str : 評価に使用した部分を除いた後の式
/// - Expression : Expressionで表現された式
fn op_mul_div(expr: &str) -> (&str, Expression) {
    let (mut expr, mut node) = op_num(expr);
    loop {
        expr = skip_space(expr);
        let op = match expr.chars().nth(0) {
            Some('*') => OperatorKind::Mul,
            Some('/') => OperatorKind::Div,
            _ => break
        };
        let (tmp_expr, right) = op_num(&expr[1..]);
        expr = tmp_expr;
        node = Expression::new(op, node, right);
    }
    (expr, node)
}

/// 数字をパースする
/// parseによって呼ばれる
///
/// # Params
/// - expr (&str) : 式
///
/// # Return
/// - &str : 評価に使用した部分を除いた後の式
/// - Expression : Expressionで表現された式
fn op_num(expr: &str) -> (&str, Expression){
    let expr = skip_space(expr);
    let (num_s, expr) = find_num(skip_space(expr));
    if num_s == "" {
        panic!("\nCannot convert to num : {}
                        ^ here", expr);
    };
    let num = num_s.parse::<i32>().unwrap();
    (expr, Expression::new_num(num))
}

/// 文頭の空白文字を読み飛ばす
///
/// # Params
/// - target (&str) : 対象文字列
///
/// # Return
/// - &str : targetの文字列スライス
fn skip_space(target: &str) -> &str {
    for (idx, c) in target.chars().enumerate() {
        match c {
            ' ' => {},
            _ => return &target[idx..]
        }
    };
    ""
}

/// 文頭にある数字を検出し、返す
///
/// # Params
/// - target (&str) : 対象文字列
///
/// # Return
/// - &str : parse::<i32>().unwrap()が可能な文字スライス
/// - &str : ↑を除いたtargetの文字列スライス
fn find_num(target: &str) -> (&str, &str) {
    for (idx, c) in target.chars().enumerate() {
        if idx == 0 && c == '-' {
            continue;
        }
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
    assert_eq!(parse("-1204").calc(), -1204);
    assert_eq!(parse("10+20").calc(), 30);
    assert_eq!(parse("30 -40").calc(), -10);
    assert_eq!(parse("1+2+ 3+4 + 5").calc(), 15);
    assert_eq!(parse("1-2 + 3-4 + 5-6").calc(), -3);
    assert_eq!(parse("-1 + 2-3+4 - 5 +6").calc(), 3);
    assert_eq!(parse("1 + 2 * 3 + 4").calc(), 11);
    assert_eq!(parse("1*2*3*4*5").calc(), 120);
    assert_eq!(parse("800/40+30").calc(), 50);
    assert_eq!(parse("1-10*90").calc(), -899);
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
