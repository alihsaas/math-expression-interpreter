use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    LParen,
    RParen,
    EndOfFile,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Modulus,
    Exponent,
}

pub fn is_addsub(token: Token) -> bool {
    token == Token::Operator(Operator::Plus) || token == Token::Operator(Operator::Minus)
}

pub fn is_muldivmod(token: Token) -> bool {
    token == Token::Operator(Operator::Mul)
        || token == Token::Operator(Operator::Div)
        || token == Token::Operator(Operator::Modulus)
}

pub fn is_exponent(token: Token) -> bool {
    token == Token::Operator(Operator::Exponent)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}
