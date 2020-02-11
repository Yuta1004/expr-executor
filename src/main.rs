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
                let ans = parse(expr).calc();
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
