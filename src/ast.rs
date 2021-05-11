use std::fmt;
use crate::token::*;

#[derive(Debug, PartialEq)]
pub struct BinOperator {
	pub left: Node,
	pub operator: Token,
	pub right: Node,
}

#[derive(Debug, PartialEq)]
pub enum Node {
	Token(Token),
	BinOperator(Box<BinOperator>),
}

impl fmt::Display for BinOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self)),
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self)),
        }
    }
}