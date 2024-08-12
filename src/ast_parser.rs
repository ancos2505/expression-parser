use std::{any::Any, fmt::Debug};

use crate::tokens::{Divide, Minus, Multiply, Plus};

#[derive(Debug, PartialEq)]
struct AST(Box<Node>);

#[derive(Debug, PartialEq)]
struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct Token(Box<dyn AstToken>);

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_any().type_id() == other.0.as_any().type_id() && self.0.eq_dyn(other.0.as_ref())
    }
}

trait AstToken: Any + Debug {
    fn as_any(&self) -> &dyn Any;
    fn eq_dyn(&self, other: &dyn AstToken) -> bool;
}

#[derive(Debug, PartialEq)]
struct NumberToken(f64);

impl AstToken for NumberToken {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn AstToken) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

impl AstToken for Plus {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn AstToken) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

impl AstToken for Minus {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn AstToken) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

impl AstToken for Multiply {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn AstToken) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

impl AstToken for Divide {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn eq_dyn(&self, other: &dyn AstToken) -> bool {
        other
            .as_any()
            .downcast_ref::<Self>()
            .map_or(false, |a| self == a)
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Divide, Minus, Multiply, Plus};

    use super::*;

    #[test]
    fn it_works() {
        let root = plus(number(3.0), number(5.0));
        let ast = AST(Box::new(root));
        dbg!(&ast);
        println!("{:?}", ast);
    }

    fn plus(left: Node, right: Node) -> Node {
        Node {
            token: Token(Box::new(Plus)),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    fn minus(left: Node, right: Node) -> Node {
        Node {
            token: Token(Box::new(Minus)),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    fn multiply(left: Node, right: Node) -> Node {
        Node {
            token: Token(Box::new(Multiply)),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    fn divide(left: Node, right: Node) -> Node {
        Node {
            token: Token(Box::new(Divide)),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
    fn number(n: f64) -> Node {
        Node {
            token: Token(Box::new(NumberToken(n))),
            left: None,
            right: None,
        }
    }
}
