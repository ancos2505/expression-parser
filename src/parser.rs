use std::{
    error::Error,
    fmt::{self, Display},
};

use crate::tokens::{Divide, LeftParen, Minus, Multiply, Number, Plus, Power, Token};

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
    tokens: Vec<Box<dyn Token>>,
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
    pub fn new(tokens: Vec<Box<dyn Token>>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> ParserResult<ASTNode> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> ParserResult<ASTNode> {
        let mut left = self.parse_term()?;

        while self.pos < self.tokens.len() {
            match self.tokens.get(self.pos) {
                Some(token) => {
                    if token.as_any().downcast_ref::<Plus>().is_some() {
                        self.pos += 1;
                        let right = self.parse_term()?;
                        left = ASTNode::Add(Box::new(left), Box::new(right));
                    } else if token.as_any().downcast_ref::<Minus>().is_some() {
                        self.pos += 1;
                        let right = self.parse_term()?;
                        left = ASTNode::Subtract(Box::new(left), Box::new(right));
                    } else {
                        break;
                    }
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
                Some(token) => {
                    if token.as_any().downcast_ref::<Multiply>().is_some() {
                        self.pos += 1;
                        let right = self.parse_factor()?;
                        left = ASTNode::Multiply(Box::new(left), Box::new(right));
                    } else if token.as_any().downcast_ref::<Divide>().is_some() {
                        self.pos += 1;
                        let right = self.parse_factor()?;
                        left = ASTNode::Divide(Box::new(left), Box::new(right));
                    } else {
                        break;
                    }
                }

                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> ParserResult<ASTNode> {
        let base = self.parse_primary()?;
        let maybe_token = self
            .tokens
            .get(self.pos)
            .map(|x| x.as_any().downcast_ref::<Power>())
            .flatten();

        if self.pos < self.tokens.len() && maybe_token == Some(&Power) {
            self.pos += 1;
            let exponent = self.parse_factor()?;
            Ok(ASTNode::Power(Box::new(base), Box::new(exponent)))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> ParserResult<ASTNode> {
        match &self.tokens.get(self.pos) {
            Some(token) => {
                if let Some(number) = token.as_any().downcast_ref::<Number>() {
                    self.pos += 1;
                    Ok(ASTNode::Number(number.0))
                } else if token.as_any().downcast_ref::<LeftParen>().is_some() {
                    self.pos += 1;
                    let expr = self.parse_expression()?;
                    self.pos += 1; // Consume right paren
                    Ok(expr)
                } else {
                    Err(ParserError("Unexpected token".into()))
                }
            }

            _ => Err(ParserError("Unexpected token".into())),
        }
    }
}
