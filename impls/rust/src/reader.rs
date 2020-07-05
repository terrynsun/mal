use std::collections::HashMap;
use regex::Regex;

use super::types::*;

struct TokenState<'a> {
    tokens: Vec<&'a str>,
    idx: usize,
}

impl<'a> TokenState<'a> {
    pub fn next(&mut self) -> MalResult<&'a str> {
        if self.idx >= self.tokens.len() {
            return Err(MalError::ParseError("unexpected EOF".to_string()))
        }

        self.idx += 1;
        Ok(self.tokens[self.idx-1])
    }

    pub fn peek(&self) -> MalResult<&'a str> {
        if self.idx >= self.tokens.len() {
            return Err(MalError::ParseError("unexpected EOF".to_string()))
        }

        Ok(self.tokens[self.idx])
    }
}

pub fn read_str(s: &str) -> MalResult<MalType> {
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

fn read_form<'a>(tokens: &mut TokenState) -> MalResult<MalType> {
    match tokens.next()? {
        "" => {
            Err(MalError::Empty)
        },
        "(" => {
            read_list(tokens, ")")
        },
        "[" => {
            read_list(tokens, "]")
        },
        "{" => {
            read_list(tokens, "}")
        },
        tok => {
            read_atom(tok)
        },
    }
}

fn read_list<'a>(tokens: &mut TokenState, end: &'static str) -> MalResult<MalType> {
    let mut items = Vec::new();
    loop {
        let next = tokens.peek()?;
        if next == ")" || next == "]" || next == "}" {
            if next == end {
                tokens.next()?;
                break;
            } else {
                return Err(MalError::ParseError(
                        format!("unbalanced parens: expected {}, got {}", end, next)));
            }
        }
        items.push(read_form(tokens)?);
    }
    match end {
        ")" => Ok(MalType::List(items)),
        "]" => Ok(MalType::Vector(items)),
        "}" => {
            let mut m = HashMap::new();
            for i in 0..items.len()/2 {
                let k = items.get(i*2).unwrap().get_string().unwrap();
                let v = items.get(i*2+1).unwrap();
                m.insert(k, v.clone());
            }

            Ok(MalType::HashMap(MalHashMap {
                map: m
            }))
        }
        _ => panic!("unexpected ending passed to read_list()"),
    }
}

fn read_atom(t: &str) -> MalResult<MalType> {
    if let Ok(n) = t.parse::<i32>() {
        Ok(MalType::Int(n))
    } else {
        match t {
            "nil"   => return Ok(MalType::Nil),
            "false" => return Ok(MalType::Bool(false)),
            "true"  => return Ok(MalType::Bool(true)),
            _ => (),
        }
        let next = t.chars().next();
        if next == Some('"') {
            parse_string(t)
        } else if next == Some(':') {
            Ok(MalType::Keyword(String::from(&t[1..])))
        } else if next == Some(';') {
            Err(MalError::Empty)
        } else {
            Ok(MalType::Symbol(String::from(t)))
        }
    }
}

fn parse_string(t: &str) -> MalResult<MalType> {
    // This is a very naive/brute force method of doing this because I struggled with the regex
    // implementation.
    let mut s = String::with_capacity(t.len());
    let chars = t.chars().peekable();

    // mini state machine to replace escaped characters
    // \n with '\n'
    // \\ with '\'
    // \" with "
    // Needs to be kept up to date with the reverse operation in printer.rs
    let mut escaped = false;
    let mut quotes = 0;
    for c in chars {
        if escaped {
            if c == 'n' {
                s.push('\n');
            } else if c == '\\' {
                s.push('\\');
            } else if c == '"' {
                s.push('"');
            } else {
                return Err(MalError::ParseError(
                    format!("unknown escaped char '{}' in {}", c, t)));
            }
            escaped = false;
        } else if c == '\\' {
            escaped = true;
        } else if c == '"' {
            if c == '"' {
                // track only unescaped quotes
                quotes += 1;
            }
        } else {
            s.push(c);
        }
    }

    if escaped {
        return Err(MalError::ParseError(
            format!("unexpected EOF: dangling escape backslash: {}", t)));
    }
    if quotes != 2 {
        return Err(MalError::ParseError(
            format!("unexpected EOF: too many or two few quotes ({}): {}", quotes, t)));
    }

    Ok(MalType::Str(s))
}
