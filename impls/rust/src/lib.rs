pub mod types;

pub mod reader;
pub mod printer;
pub mod env;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
