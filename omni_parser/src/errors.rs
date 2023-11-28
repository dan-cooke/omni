use std::{
    convert::Infallible,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug, PartialEq, Clone)]
pub enum LexicalError {
    InvalidToken(String),
    ParseFloatError,
    ParseIntError,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorDetails {
    pub message: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    RequiredField(ErrorDetails),
    TypeError(ErrorDetails),
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
impl From<ParseFloatError> for LexicalError {
    fn from(_: ParseFloatError) -> Self {
        Self::ParseFloatError
    }
}
