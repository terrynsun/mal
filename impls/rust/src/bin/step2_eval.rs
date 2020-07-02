use std::io;
use std::io::Write;
use std::collections::HashMap;

use mal::types::*;

fn read(s: String) -> MalResult<MalType> {
    let trimmed = s.trim();
    mal::reader::read_str(&trimmed)
}

fn eval_ast(expr: MalType, env: &HashMap<String, MalType>) -> MalType {
    match expr {
        MalType::Symbol(s) => {
            env.get(&s).unwrap_or(&MalType::Nil).clone()
        }
        MalType::List(list) => {
            MalType::List(list.into_iter().map(|e| eval_ast(e, env)).collect())
        }
        _ => expr,
    }
}

fn eval(expr: MalType, env: &HashMap<String, MalType>) -> MalType {
    eval_ast(expr, env)
}

fn print(expr: MalType) -> String {
    mal::printer::pr_str(expr, true)
}

fn rep(s: String) -> MalResult<String> {
    let mut env = HashMap::new();
    env.insert(String::from("foo"), MalType::Int(0));

    let a = read(s)?;
    let b = eval(a, &env);
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
