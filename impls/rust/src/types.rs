pub type MalList = Vec<MalType>;

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Bool(bool),
    Int(i32),
    Keyword(String),
    Symbol(String),
    Str(String),
    List(MalList),
    Fun(fn(MalList) -> MalType),
}

#[derive(Debug, Clone)]
pub enum MalError {
    Empty, // not an error; either comment or blank input
    ParseError(String),
}

pub type MalResult<T> = Result<T, MalError>;
