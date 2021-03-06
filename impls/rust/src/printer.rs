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
        MalType::Fun(_) => "TODO: cannot print fn pointers".to_string(),
        MalType::Bool(b) => b.to_string(),
        MalType::Int(i) => i.to_string(),
        MalType::Symbol(s) => s,
        MalType::Keyword(s) => format!(":{}", s),
        MalType::Str(s) => {
            if print_readably {
                format!("\"{}\"", escape_str(&s))
            } else {
                format!("\"{}\"", s)
            }
        },
        MalType::List(exprs) => {
            let acc: Vec<String> = exprs.into_iter()
                .map(|e| pr_str(e, print_readably)).collect();
            format!("({})", acc.join(" "))
        }
        MalType::Vector(exprs) => {
            let acc: Vec<String> = exprs.into_iter()
                .map(|e| pr_str(e, print_readably)).collect();
            format!("[{}]", acc.join(" "))
        }
        MalType::HashMap(map) => {
            let mut acc = Vec::new();

            for (k, v) in map.map {
                acc.push(format!("{} {}", k, pr_str(v, print_readably)));
            }

            format!("{{{}}}", acc.join(" "))
        }
    }
}
