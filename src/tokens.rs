use std::{
    any::Any,
    fmt::{Debug, Display},
};

use crate::ast_parser::AstToken;

pub(crate) trait Token: Any + Debug {
    // fn as_str(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn to_token(self) -> Box<dyn Token>;
}

impl Token for Number {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for Plus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for Minus {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for Multiply {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for Divide {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for Power {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for LeftParen {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}
impl Token for RightParen {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn to_token(self) -> Box<dyn Token> {
        Box::new(self)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Number(pub f64);
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Plus;

impl Plus {
    pub const fn op_name() -> &'static str {
        "plus"
    }
    pub const fn as_str() -> &'static str {
        "+"
    }
}
impl Display for Plus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::op_name())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Minus;

impl Minus {
    pub const fn op_name() -> &'static str {
        "minus"
    }
    pub const fn as_str() -> &'static str {
        "-"
    }
}
impl Display for Minus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::op_name())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Multiply;

impl Multiply {
    pub const fn op_name() -> &'static str {
        "multiply"
    }
    pub const fn as_str() -> &'static str {
        "*"
    }
}
impl Display for Multiply {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::op_name())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Divide;

impl Divide {
    pub const fn op_name() -> &'static str {
        "divide"
    }
    pub const fn as_str() -> &'static str {
        "/"
    }
}
impl Display for Divide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::op_name())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Power;

impl Power {
    pub const fn op_name() -> &'static str {
        "power"
    }
    pub const fn as_str() -> &'static str {
        "^"
    }
}
impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::op_name())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LeftParen;
impl LeftParen {
    pub const fn as_str() -> &'static str {
        "("
    }
}
impl Display for LeftParen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RightParen;
impl RightParen {
    pub const fn as_str() -> &'static str {
        ")"
    }
}
impl Display for RightParen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::as_str())
    }
}
