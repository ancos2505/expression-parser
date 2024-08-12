use std::{error::Error, fmt::Display};

use crate::{lexer::LexerError, parser::ParserError};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Lexer(LexerError),
    Parser(ParserError),
}
impl From<LexerError> for AppError {
    fn from(error: LexerError) -> Self {
        Self::Lexer(error)
    }
}
impl From<ParserError> for AppError {
    fn from(error: ParserError) -> Self {
        Self::Parser(error)
    }
}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "Error".to_string();
        let s = match self {
            AppError::Lexer(err) => format!("(Lexer): {err}"),
            AppError::Parser(err) => format!("(Parser): {err}"),
        };
        output.push_str(s.as_str());

        write!(f, "{output}")
    }
}

impl Error for AppError {}
