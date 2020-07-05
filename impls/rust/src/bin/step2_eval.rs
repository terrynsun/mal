use std::io;
use std::io::Write;
use std::collections::HashMap;

use mal::types::*;

fn read(s: String) -> MalResult<MalType> {
    let trimmed = s.trim();
    mal::reader::read_str(&trimmed)
}

/// Simplifies an expression
/// - looks up a symbol in the environment.
/// - resolves each element in a list.
fn eval_ast(expr: MalType, env: &HashMap<String, MalType>) -> MalType {
    //println!("-- eval_ast() {:?}", expr);
    match expr {
        // Look up a variable in environment
        MalType::Symbol(s) => {
            env.get(&s).unwrap_or(&MalType::Nil).clone()
        }
        // Simplify each element in a list
        MalType::List(list) => {
            MalType::List(list.into_iter().map(|e| eval(e, env)).collect())
        }
        _ => expr,
    }
}

// Resolve an expr to a final value.
fn eval(expr: MalType, env: &HashMap<String, MalType>) -> MalType {
    // Simplify list, then resolve by applying the function (first elt) to all other elts.
    if let MalType::List(list) = expr {
        if list.is_empty() {
            // Ownership question: it would be better to just return `expr` here.
            MalType::List(Vec::new())
        } else {
            let simplified = eval_ast(MalType::List(list), env);
            if let MalType::List(list) = simplified {
                let op = list.get(0).unwrap();
                if let MalType::Fun(f) = op {
                    f(list[1..].to_vec())
                } else {
                    MalType::List(list)
                }
            } else {
                panic!("eval_ast(MalList) should always yield MalList");
            }
        }
    } else {
        // All other types are resolved by eval_ast.
        eval_ast(expr, env)
    }
}

fn print(expr: MalType) -> String {
    mal::printer::pr_str(expr, true)
}

fn default_add(args: MalList) -> MalType {
    MalType::Int(
        args.into_iter().fold(0, |acc, e| {
            if let MalType::Int(i) = e { acc + i } else { acc }
        })
    )
}

fn default_sub(args: MalList) -> MalType {
    // So far assumes there are only two arguments, and panics otherwise.
    if args.len() == 2 {
        if let Some(MalType::Int(i1)) = args.get(0) {
            if let Some(MalType::Int(i2)) = args.get(1) {
                return MalType::Int(i1 - i2);
            }
        }
    }
    panic!("(-) has invalid arguments: {:?}", args);
}

fn default_mul(args: MalList) -> MalType {
    MalType::Int(
        args.into_iter().fold(1, |acc, e| {
            if let MalType::Int(i) = e { acc * i } else { acc }
        })
    )
}

fn default_div(args: MalList) -> MalType {
    // So far assumes there are only two arguments, and panics otherwise.
    if args.len() == 2 {
        if let Some(MalType::Int(i1)) = args.get(0) {
            if let Some(MalType::Int(i2)) = args.get(1) {
                return MalType::Int(i1 / i2);
            }
        }
    }
    panic!("(/) has invalid arguments: {:?}", args);
}

fn rep(s: String) -> MalResult<String> {
    let mut env = HashMap::new();
    env.insert(String::from("foo"), MalType::Int(0));
    env.insert(String::from("+"), MalType::Fun(default_add));
    env.insert(String::from("-"), MalType::Fun(default_sub));
    env.insert(String::from("*"), MalType::Fun(default_mul));
    env.insert(String::from("/"), MalType::Fun(default_div));

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
        Err(MalError::Empty) => (),
        Err(MalError::ParseError(e)) => println!("error: {:?}", e),
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
