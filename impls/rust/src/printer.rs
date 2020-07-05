use super::types::*;
use regex::Regex;

fn escape_str(s: &str) -> String {
    let re = Regex::new(r"(\\)").unwrap();
    let result = re.replace_all(s, r#"\\"#);

    let re = Regex::new(r#"(\n)"#).unwrap();
    let result = re.replace_all(&result, "\\n");

    let re = Regex::new(r#"(")"#).unwrap();
    let result = re.replace_all(&result, "\\\"");
    result.into_owned()
}

pub fn pr_str(expr: MalType, print_readably: bool) -> String {
    match expr {
        MalType::Nil => String::from("nil"),
        MalType::Fun(_) => format!("TODO: cannot print fn pointers"),
        MalType::Int(i) => i.to_string(),
        MalType::Symbol(s) => s,
        MalType::Str(s) => {
            if print_readably {
                format!("\"{}\"", escape_str(&s))
            } else {
                format!("\"{}\"", s)
            }
        },
        MalType::List(exprs) => {
            let mut acc = Vec::new();
            for e in exprs {
                acc.push(pr_str(e, print_readably))
            }
            format!("({})", acc.join(" "))
        }
    }
}
