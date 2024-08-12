use std::{any::Any, fmt::Debug};

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

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Plus;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Minus;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Multiply;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Divide;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Power;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LeftParen;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RightParen;
