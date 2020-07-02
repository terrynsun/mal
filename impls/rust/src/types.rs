pub type MalList = Vec<MalType>;

#[derive(Debug, Clone)]
pub enum MalType {
    Int(i32),
    Symbol(String),
    Str(String),
    List(MalList)
}

#[derive(Debug, Clone)]
pub enum MalError {
    ParseError(String)
}

pub type MalResult<T> = Result<T, MalError>;
