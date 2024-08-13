use std::{
    any::Any,
    collections::VecDeque,
    fmt::{format, Debug, Display},
};

use crate::tokens::{Divide, Minus, Multiply, Number, Plus, Power};

#[derive(Debug, PartialEq)]
struct AST {
    tree: Box<Node>,
    depth: usize,
}

impl Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tree)
    }
}
#[derive(Debug, PartialEq)]
struct Node {
    token: Token,
    depth: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        output.push_str(
            format!(
                "{}",
                self.token
                    .get_operation()
                    .unwrap_or(self.token.to_string().as_str())
            )
            .as_str(),
        );

        if let Some(node) = &self.left {
            output.push_str(format!("{node}").as_str());
        }

        if let Some(node) = &self.right {
            output.push_str(format!("{node}").as_str());
        }
        write!(f, "({output})")
    }
}

#[derive(Debug)]
struct Token(Box<dyn AstToken>);
impl Token {
    pub fn get_operation(&self) -> Option<&'static str> {
        if self.0.as_any().downcast_ref::<Plus>().is_some() {
            Some(Plus::op_name())
        } else if self.0.as_any().downcast_ref::<Minus>().is_some() {
            Some(Minus::op_name())
        } else if self.0.as_any().downcast_ref::<Multiply>().is_some() {
            Some(Multiply::op_name())
        } else if self.0.as_any().downcast_ref::<Divide>().is_some() {
            Some(Divide::op_name())
        } else if self.0.as_any().downcast_ref::<Power>().is_some() {
            Some(Power::op_name())
        } else {
            None
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_any().type_id() == other.0.as_any().type_id() && self.0.eq_dyn(other.0.as_ref())
    }
}

pub(crate) trait AstToken: Any + Debug + Display {
    fn as_any(&self) -> &dyn Any;
    fn eq_dyn(&self, other: &dyn AstToken) -> bool;
}

impl AstToken for Number {
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

impl AstToken for Power {
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

fn plus(left: Node, right: Node) -> Node {
    Node {
        token: Token(Box::new(Plus)),
        depth: left.depth.max(right.depth) + 1,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}
fn minus(left: Node, right: Node) -> Node {
    Node {
        token: Token(Box::new(Minus)),
        depth: left.depth.max(right.depth) + 1,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}
fn multiply(left: Node, right: Node) -> Node {
    Node {
        token: Token(Box::new(Multiply)),
        depth: left.depth.max(right.depth) + 1,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}
fn divide(left: Node, right: Node) -> Node {
    Node {
        token: Token(Box::new(Divide)),
        depth: left.depth.max(right.depth) + 1,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}

fn power(base: Node, expoent: Node) -> Node {
    Node {
        token: Token(Box::new(Power)),
        depth: base.depth.max(expoent.depth) + 1,
        left: Some(Box::new(base)),
        right: Some(Box::new(expoent)),
    }
}

fn number(n: f64) -> Node {
    Node {
        token: Token(Box::new(Number(n))),
        depth: 1,
        left: None,
        right: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3 + 4 * (2 + 1)^2";
        let root = plus(
            number(3.0),
            multiply(
                number(4.0),
                power(plus(number(2.0), number(1.0)), number(2.0)),
            ),
        );

        let ast = AST {
            depth: root.depth,
            tree: Box::new(root),
        };

        dbg!(&ast);

        dbg!(&input);
        println!("{}", ast);
    }
}
