use regex::Regex;

use super::types::*;

struct TokenState<'a> {
    tokens: Vec<&'a str>,
    idx: usize,
}

impl<'a> TokenState<'a> {
    pub fn next(&mut self) -> &'a str {
        if self.idx >= self.tokens.len() {
            panic!("unexpected EOF");
        }

        self.idx += 1;
        self.tokens[self.idx-1]
    }

    pub fn peek(&self) -> &'a str {
        if self.idx >= self.tokens.len() {
            panic!("unexpected EOF");
        }

        self.tokens[self.idx]
    }
}

pub fn read_str(s: &str) -> MalType {
    let mut tokens = tokenize(s);
    read_form(&mut tokens)
}

fn tokenize(s: &str) -> TokenState {
    // The following regular expression (PCRE) will match all mal tokens.
    // [\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)
    let regex_str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#;
    let tokens_re = Regex::new(regex_str).unwrap();

    let mut acc = Vec::new();
    for caps in tokens_re.captures_iter(s) {
        acc.push(caps.get(1).unwrap().as_str().trim())
    }
    TokenState {
        tokens: acc,
        idx: 0
    }
}

fn read_form<'a>(tokens: &mut TokenState) -> MalType {
    match tokens.next() {
        "(" => {
            read_list(tokens)
        },
        tok => {
            read_atom(tok)
        },
    }
}

fn read_list<'a>(tokens: &mut TokenState) -> MalType {
    let mut items = Vec::new();
    loop {
        if tokens.peek() == ")" {
            tokens.next();
            break;
        } else {
            items.push(read_form(tokens));
        }
    }
    MalType::List(items)
}

fn read_atom(t: &str) -> MalType {
    if let Ok(n) = t.parse::<i32>() {
        MalType::Int(n)
    } else {
        MalType::Symbol(String::from(t))
    }
}
