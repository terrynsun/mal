use std::io;
use std::io::Write;

use mal::env::*;
use mal::types::*;

fn read(s: String) -> MalResult<MalType> {
    let trimmed = s.trim();
    mal::reader::read_str(&trimmed)
}

/// Simplifies an expression
/// - looks up a symbol in the environment.
/// - resolves each element in a list.
fn eval_ast(expr: MalType, env: &mut MalEnv) -> MalResult<MalType> {
    //println!("-- eval_ast() {:?}", expr);
    match expr {
        // Look up a variable in environment
        MalType::Symbol(s) => {
            // TODO: return not-found error instead of nil
            if let Some(val) = env.get(&s) {
                return Ok(val.clone());
            } else {
                return Err(MalError::RuntimeError(format!("{} not found in environment", s)));
            }
        }
        // Simplify each element in a list
        MalType::List(list) => {
            let mut acc = Vec::new();
            for e in list {
                acc.push(eval(e, env)?);
            }
            Ok(MalType::List(acc))
        }
        _ => Ok(expr),
    }
}

fn update_env(key: &MalType, val: MalType, env: &mut MalEnv) -> MalResult<()>{
    if let MalType::Symbol(s) = key {
        env.set(String::from(s), val);
        Ok(())
    } else {
        Err(MalError::RuntimeError(
                format!("invalid def: {:?} = {:?}", key, val)))
    }
}

// Resolve an expr to a final value.
fn eval(expr: MalType, env: &mut MalEnv) -> MalResult<MalType> {
    // Simplify list, then resolve by applying the function (first elt) to all other elts.
    if let MalType::List(list) = expr {
        if list.is_empty() {
            // Ownership question: it would be better to just return `expr` here.
            Ok(MalType::List(Vec::new()))
        } else {
            // Check first elt to see if it's a special.
            let op = list.get(0).unwrap();
            if let MalType::Symbol(s) = op {
                match &s[..] {
                    "def!" => {
                        let arg1 = list.get(1).unwrap();
                        let arg2 = list.get(2).unwrap();
                        let val = eval(arg2.clone(), env)?;
                        update_env(arg1, val.clone(), env)?;
                        return Ok(val.clone());
                    },
                    "let*" => {
                        let mut inner = MalEnv::new(Some(env));
                        let arg1 = list.get(1).unwrap();
                        if let MalType::List(list) = arg1 {
                            println!(">{:?}", list);
                            for i in 0..(list.len()/2) {
                                let arg1 = list.get(i*2).unwrap();
                                let arg2 = list.get(i*2 + 1).unwrap();
                                let val = eval(arg2.clone(), &mut inner)?;
                                update_env(arg1, val.clone(), &mut inner)?;
                                println!("{:?} = {:?}", arg1, val);
                            }
                        }

                        let final_expr = list.get(2).unwrap();
                        return eval(final_expr.clone(), &mut inner)
                    },
                    _ => (),
                }
            }

            let simplified = eval_ast(MalType::List(list), env)?;
            if let MalType::List(list) = simplified {
                let op = list.get(0).unwrap();
                if let MalType::Fun(f) = op {
                    f(list[1..].to_vec())
                } else {
                    Ok(MalType::List(list))
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

fn rep(s: String, env: &mut MalEnv) -> MalResult<String> {
    let a = read(s)?;
    let b = eval(a, env)?;
    let c = print(b);
    Ok(c)
}

fn repl_loop(env: &mut MalEnv) -> bool {
    print!("user> ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error reading line");

    if input.len() == 0 {
        println!();
        return false;
    }

    match rep(input, env) {
        Ok(output) => println!("{}", output),
        Err(MalError::Empty) => (),
        Err(e) => println!("error: {:?}", e),
    }

    true
}

fn main() {
    let mut env = MalEnv::default();
    loop {
        if !repl_loop(&mut env) {
            break;
        }
    }
}
