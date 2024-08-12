use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::{
    location::Location,
    tokens::{Divide, LeftParen, Minus, Multiply, Number, Plus, Power, RightParen, Token},
};

type LexerResult<T> = Result<T, LexerError>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    length: usize,
    location: Location,
}

#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub location: Location,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error:[{}] in line {} at column {}",
            self.message,
            self.location.line(),
            self.location.col()
        )
    }
}

impl Error for LexerError {}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            length: input.chars().count(),
            location: Location::new(),
        }
    }

    pub fn next_token(&mut self) -> LexerResult<Option<Box<dyn Token>>> {
        self.skip_whitespace()?;

        let (_, after) = self
            .input
            .split_at_checked(self.location.index())
            .ok_or(LexerError {
                message: "Error on reading the end of input".into(),
                location: self.location,
            })?;

        match after.chars().next() {
            Some(ch) => {
                let token = match ch {
                    '0'..='9' => self.read_number(),
                    '+' => {
                        self.location.advance(ch);
                        Ok(Plus.to_token())
                    }
                    '-' => {
                        self.location.advance(ch);
                        Ok(Minus.to_token())
                    }
                    '*' => {
                        self.location.advance(ch);
                        Ok(Multiply.to_token())
                    }
                    '/' => {
                        self.location.advance(ch);
                        Ok(Divide.to_token())
                    }
                    '(' => {
                        self.location.advance(ch);
                        Ok(LeftParen.to_token())
                    }
                    ')' => {
                        self.location.advance(ch);
                        Ok(RightParen.to_token())
                    }
                    '^' => {
                        self.location.advance(ch);
                        Ok(Power.to_token())
                    }
                    _ => Err(LexerError {
                        message: format!("Unexpected character: {}", ch),
                        location: self.location,
                    }),
                };

                token.map(Some)
            }
            None => {
                self.location.backtrack();
                Ok(None)
            }
        }
    }

    fn read_number(&mut self) -> LexerResult<Box<dyn Token>> {
        let mut number = String::new();
        let start_location = self.location;
        let mut has_decimal = false;

        let (_, after) = self
            .input
            .split_at_checked(self.location.index())
            .ok_or(LexerError {
                message: "Error on reading the end of input".into(),
                location: self.location,
            })?;
        let mut s = after.chars();
        while let Some(ch) = s.next() {
            if ch.is_digit(10) {
                number.push(ch);
                self.location.advance(ch);
            } else if ch == '.' && !has_decimal {
                number.push(ch);
                has_decimal = true;
                self.location.advance(ch);
            } else {
                break;
            }
        }

        if number.ends_with('.') {
            return Err(LexerError {
                message: format!("Invalid number format: {}", number),
                location: start_location,
            });
        }

        let token = number.parse::<f64>().map(Number).map_err(|_| LexerError {
            message: format!("Invalid number: {}", number),
            location: start_location,
        })?;

        Ok(token.to_token())
    }

    fn skip_whitespace(&mut self) -> LexerResult<()> {
        let (_, after) = self
            .input
            .split_at_checked(self.location.index())
            .ok_or(LexerError {
                message: "Error on reading the end of input".into(),
                location: self.location,
            })?;
        let mut s = after.chars();
        while let Some(ch) = s.next() {
            if ch.is_whitespace() {
                self.location.advance(ch);
            } else {
                break;
            }
        }
        Ok(())
    }
}
