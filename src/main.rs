use expr_executor::parser::parse::parse;

use std::io::{self, Write};

fn main() {
    let mut cnt = 0;
    loop {
        // prompt
        print!("inp[{}] > ", cnt);
        io::stdout().flush().unwrap();
        cnt += 1;

        // command
        match &read_line()[..] {
            "exit" => break,
            _ => continue
        }
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
