use std::io;
use std::io::Write;

use mal::types::*;

fn read(s: String) -> MalResult<MalType> {
    let trimmed = s.trim();
    mal::reader::read_str(&trimmed)
}

fn eval(expr: MalType) -> MalType {
    expr
}

fn print(expr: MalType) -> String {
    mal::printer::pr_str(expr, true)
}

fn rep(s: String) -> MalResult<String> {
    let a = read(s)?;
    let b = eval(a);
    let c = print(b);
    Ok(c)
}

fn repl_loop() -> bool {
    print!("user> ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading line");

    if input.len() == 0 {
        println!();
        return false;
    }

    match rep(input) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("error: {:?}", e),
    }

    true
}

fn main() {
    loop {
        if !repl_loop() {
            break;
        }
    }
}
