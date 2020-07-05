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
    Fun(fn(MalList) -> MalResult<MalType>),
}

impl MalType {
    pub fn is_list(&self) -> bool {
        if let &MalType::List(_) = self {
            true
        } else {
            false
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
