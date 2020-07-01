use super::types::*;

pub fn pr_str(expr: MalType) -> String {
    match expr {
        MalType::Int(i) => i.to_string(),
        MalType::Symbol(s) => s,
        MalType::List(exprs) => {
            let mut acc = Vec::new();
            for e in exprs {
                acc.push(pr_str(e))
            }
            format!("({})", acc.join(" "))
        }
    }
}
