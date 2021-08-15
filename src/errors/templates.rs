use crate::errors::Error;

pub enum Errors {}

impl Errors {
    pub fn unknown_keyword(line: usize, at: usize, word: String) -> Error {
        Error {
            line,
            at,
            message: format!("Keyword {} is unknown.", word),
            symbol: None,
        }
    }
}
