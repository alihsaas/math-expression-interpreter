use crate::token::*;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BinOperator {
    pub left: Node,
    pub operator: Token,
    pub right: Node,
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperator {
    pub operator: Token,
    pub expression: Node,
}

#[derive(Debug, PartialEq)]
pub enum Node {
    Token(Token),
    BinOperator(Box<BinOperator>),
    UnaryOperator(Box<UnaryOperator>),
}

impl fmt::Display for BinOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
