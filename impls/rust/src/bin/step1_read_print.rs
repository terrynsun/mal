use std::io;
use std::io::Write;

use mal::types::*;

fn read(s: String) -> MalType {
    let trimmed = s.trim();
    mal::reader::read_str(&trimmed)
}

fn eval(expr: MalType) -> MalType {
    expr
}

fn print(expr: MalType) -> String {
    mal::printer::pr_str(expr)
}

fn rep(s: String) -> String {
    print(eval(read(s)))
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

    let output = rep(input);

    println!("{}", output);
    true
}

fn main() {
    loop {
        if !repl_loop() {
            break;
        }
    }
}
