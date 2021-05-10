use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    LParen,
    RParen,
    EndOfFile,
    Unknown,
    Whitespace,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn is_addsub(token: Token) -> bool {
    token == Token::Operator(Operator::Add) || token == Token::Operator(Operator::Sub)
}

pub fn is_muldiv(token: Token) -> bool {
    token == Token::Operator(Operator::Mul) || token == Token::Operator(Operator::Div)
}

pub fn is_whitespace(token: Token) -> bool {
    token == Token::Whitespace
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self)),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => f.write_str(&format!("{:?}", self)),
        }
    }
}
