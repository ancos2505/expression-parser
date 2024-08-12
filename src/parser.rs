use crate::Token;

use std::{
    error::Error,
    fmt::{self, Display},
};

type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug)]
pub enum ASTNode {
    Number(f64),
    Add(Box<ASTNode>, Box<ASTNode>),
    Subtract(Box<ASTNode>, Box<ASTNode>),
    Multiply(Box<ASTNode>, Box<ASTNode>),
    Divide(Box<ASTNode>, Box<ASTNode>),
    Power(Box<ASTNode>, Box<ASTNode>),
}

impl Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Number(n) => write!(f, "{}", n),
            ASTNode::Add(l, r) => write!(f, "({} + {})", l, r),
            ASTNode::Subtract(l, r) => write!(f, "({} - {})", l, r),
            ASTNode::Multiply(l, r) => write!(f, "({} * {})", l, r),
            ASTNode::Divide(l, r) => write!(f, "({} / {})", l, r),
            ASTNode::Power(l, r) => write!(f, "({} ^ {})", l, r),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
pub struct ParserError(pub String);
impl Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParserError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> ParserResult<ASTNode> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParserResult<ASTNode> {
        let mut left = self.parse_term()?;

        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(Token::Plus) => {
                    self.pos += 1;
                    let right = self.parse_term()?;
                    left = ASTNode::Add(Box::new(left), Box::new(right));
                }
                Some(Token::Minus) => {
                    self.pos += 1;
                    let right = self.parse_term()?;
                    left = ASTNode::Subtract(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> ParserResult<ASTNode> {
        let mut left = self.parse_factor()?;

        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(Token::Multiply) => {
                    self.pos += 1;
                    let right = self.parse_factor()?;
                    left = ASTNode::Multiply(Box::new(left), Box::new(right));
                }
                Some(Token::Divide) => {
                    self.pos += 1;
                    let right = self.parse_factor()?;
                    left = ASTNode::Divide(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> ParserResult<ASTNode> {
        let base = self.parse_primary()?;

        if self.pos < self.tokens.len() && self.tokens.get(self.pos) == Some(&Token::Power) {
            self.pos += 1;
            let exponent = self.parse_factor()?;
            Ok(ASTNode::Power(Box::new(base), Box::new(exponent)))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> ParserResult<ASTNode> {
        match &self.tokens.get(self.pos) {
            Some(Token::Number(n)) => {
                self.pos += 1;
                Ok(ASTNode::Number(*n))
            }
            Some(Token::LeftParen) => {
                self.pos += 1;
                let expr = self.parse_expression()?;
                self.pos += 1; // Consume right paren
                Ok(expr)
            }
            _ => Err(ParserError("Unexpected token".into())),
        }
    }
}
