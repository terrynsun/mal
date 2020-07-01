pub type MalList = Vec<MalType>;

pub enum MalType {
    Int(i32),
    Symbol(String),
    List(MalList)
}
