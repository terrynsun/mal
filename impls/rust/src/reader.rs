use regex::Regex;

use super::types::*;

struct TokenState<'a> {
    tokens: Vec<&'a str>,
    idx: usize,
}

impl<'a> TokenState<'a> {
    pub fn next(&mut self) -> Result<&'a str, ()> {
        if self.idx >= self.tokens.len() {
            println!("unexpected EOF");
            return Err(())
        }

        self.idx += 1;
        Ok(self.tokens[self.idx-1])
    }

    pub fn peek(&self) -> Result<&'a str, ()> {
        if self.idx >= self.tokens.len() {
            println!("unexpected EOF");
            return Err(())
        }

        Ok(self.tokens[self.idx])
    }
}

pub fn read_str(s: &str) -> Result<MalType, ()> {
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

fn read_form<'a>(tokens: &mut TokenState) -> Result<MalType, ()> {
    match tokens.next()? {
        "(" => {
            read_list(tokens)
        },
        tok => {
            read_atom(tok)
        },
    }
}

fn read_list<'a>(tokens: &mut TokenState) -> Result<MalType, ()> {
    let mut items = Vec::new();
    loop {
        if tokens.peek()? == ")" {
            tokens.next()?;
            break;
        } else {
            items.push(read_form(tokens)?);
        }
    }
    Ok(MalType::List(items))
}

fn read_atom(t: &str) -> Result<MalType, ()> {
    if let Ok(n) = t.parse::<i32>() {
        Ok(MalType::Int(n))
    } else {
        let mut chars = t.chars();
        if chars.next() == Some('"') {
            parse_string(t)
        } else {
            Ok(MalType::Symbol(String::from(t)))
        }
    }
}

fn parse_string(t: &str) -> Result<MalType, ()> {
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
                println!("unknown escaped character '{}' in {}", c, t);
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
        println!("unexpected EOF: unfinished escape sequence: {}", t);
        return Err(());
    }
    if quotes != 2 {
        println!("unexpected EOF: too many or two few quotes ({}): {}", quotes, t);
        return Err(());
    }

    Ok(MalType::Str(s))
}
