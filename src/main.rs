use expr_executor::parser::parse::parse;

use std::io::{self, Write};
use std::vec::Vec;

fn main() {
    let mut cnt = 0;
    let mut history: Vec<i32> = Vec::new();

    loop {
        // prompt
        print!("in [{}] > ", cnt);
        io::stdout().flush().unwrap();

        // command
        match &read_line()[..] {
            "exit" => break,
            "" => {
                cnt += 1;
                continue;
            },
            expr @ _ => {
                let expr = pre_process(expr, &history);
                let ans = parse(&expr).calc();
                history.push(ans);
                println!("out[{}] > {}", cnt, ans);
            }
        }
        cnt += 1;
    }
}

/// 標準入力から1行読み込んで返す
///
/// # Params
///
/// # Return
/// - String : 入力された文字列
fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf.trim().parse().ok().unwrap()
}

/// 変数文字列を過去の計算結果で置換する(前処理)
///
/// # Params
/// - expr(&str) : 式
/// - history(vec<i32>) : 過去の計算結果
///
/// # Return
/// - String : 前処理後の処理
fn pre_process(expr: &str, history: &Vec<i32>) -> String {
    let mut expr_str = expr.to_string();
    for (idx, num) in history.iter().enumerate() {
        let var_str = &format!("*{}", idx);
        let num_str = &format!("{}", num);
        expr_str = expr_str.replace(var_str, num_str);
    }
    expr_str
}
