use std::{convert::Infallible, num::ParseIntError};

#[derive(Debug, PartialEq, Clone)]
pub enum LexicalError {
    InvalidToken(String),
    ParseIntError,
    ParseStringError,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidDefinition(String),
    DuplicateDefinition(String),
    UndefinedDefinition(String),
}

impl Default for LexicalError {
    fn default() -> Self {
        Self::InvalidToken("".to_string())
    }
}

impl From<Infallible> for LexicalError {
    fn from(_: Infallible) -> Self {
        Self::InvalidToken("".to_string())
    }
}
impl From<ParseIntError> for LexicalError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseIntError
    }
}
