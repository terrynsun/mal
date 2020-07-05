use std::collections::HashMap;

pub type MalList = Vec<MalType>;

#[derive(Debug, Clone)]
pub struct MalHashMap {
    pub map: HashMap<String, MalType>,
}

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Keyword(String),
    Symbol(String),
    Str(String),
    List(MalList),
    Vector(MalList),
    HashMap(MalHashMap),
    Fun(fn(MalList) -> MalResult<MalType>),
}

impl MalType {
    pub fn is_list(&self) -> bool {
        if let MalType::List(_) = *self {
            true
        } else {
            false
        }
    }

    // Returns the string from a String or Keyword
    pub fn get_string(&self) -> Option<String> {
        match self {
            MalType::Str(s) | MalType::Keyword(s) => Some(s.clone()),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub enum MalError {
    Empty, // not an error; either comment or blank input
    NotFoundError,
    ParseError(String),
    RuntimeError(String),
}

pub type MalResult<T> = Result<T, MalError>;
