pub type MalList = Vec<MalType>;

#[derive(Debug, Clone)]
pub enum MalType {
    Nil,
    Int(i32),
    Symbol(String),
    Str(String),
    List(MalList),
    Fun(fn(MalList) -> MalType),
}

#[derive(Debug, Clone)]
pub enum MalError {
    ParseError(String)
}

pub type MalResult<T> = Result<T, MalError>;
