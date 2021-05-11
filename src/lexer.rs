use crate::token::{Operator, Token};
use std::{collections::VecDeque, iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Lexer<'a> {
    tokens: VecDeque<Token>,
    char_iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            tokens: VecDeque::new(),
            char_iter: text.trim().chars().peekable(),
        }
    }

    pub fn next(&mut self) -> Token {
        self.tokens.pop_back().unwrap_or(Token::Unknown)
    }

    pub fn peek(&self) -> Token {
        *self.tokens.back().unwrap_or(&Token::Unknown)
    }

    pub fn lex(&mut self) -> &VecDeque<Token> {
        while let Some(c) = self.char_iter.next() {
            match c {
                '0'..='9' => {
                    let float = self.parse_float(&c.to_string());
                    self.tokens.push_front(Token::Number(float))
                }
                '+' => self.tokens.push_front(Token::Operator(Operator::Plus)),
                '-' => self.tokens.push_front(Token::Operator(Operator::Minus)),
                '*' => self.tokens.push_front(Token::Operator(Operator::Mul)),
                '/' => self.tokens.push_front(Token::Operator(Operator::Div)),
                '(' => self.tokens.push_front(Token::LParen),
                ')' => self.tokens.push_front(Token::RParen),
                ' ' => self.tokens.push_front(Token::Whitespace),
                _ => self.tokens.push_front(Token::Unknown),
            }
        }

        self.tokens.push_front(Token::EndOfFile);

        dbg!(&self.tokens);

        &self.tokens
    }
    fn parse_float(&mut self, text: &str) -> f64 {
        let mut buffer = text.to_string();

        while let Some(c) = self.char_iter.peek() {
            match c {
                '0'..='9' | '.' => buffer.push(self.char_iter.next().unwrap()),
                _ => break,
            }
        }

        buffer.parse().expect("Failed to parse float")
    }
}
