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
        self.tokens.pop_front().unwrap_or(Token::Unknown)
    }

    pub fn peek(&self) -> Token {
        *self.tokens.front().unwrap_or(&Token::Unknown)
    }

    pub fn lex(&mut self) -> &VecDeque<Token> {
        while let Some(c) = self.char_iter.next() {
            let peek = *self.char_iter.peek().unwrap_or(&'\0');
            match c {
                '0'..='9' => {
                    let float = self.parse_float(&c.to_string());
                    self.tokens.push_back(Token::Number(float))
                }
                '+' => self.tokens.push_back(Token::Operator(Operator::Plus)),
                '-' => self.tokens.push_back(Token::Operator(Operator::Minus)),
                '*' => {
                    let token = self.match_char(
                        peek,
                        '*',
                        Token::Operator(Operator::Exponent),
                        Token::Operator(Operator::Mul),
                    );
                    self.tokens.push_back(token)
                }
                '/' => self.tokens.push_back(Token::Operator(Operator::Div)),
                '%' => self.tokens.push_back(Token::Operator(Operator::Modulus)),
                '(' => self.tokens.push_back(Token::LParen),
                ')' => self.tokens.push_back(Token::RParen),
                _ => (),
            }
        }

        self.tokens.push_back(Token::EndOfFile);

        dbg!(&self.tokens);

        &self.tokens
    }

    fn match_char(
        &mut self,
        peek: char,
        match_char: char,
        matched: Token,
        unmatched: Token,
    ) -> Token {
        if peek == match_char {
            self.char_iter.next();
            matched
        } else {
            unmatched
        }
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

#[test]
fn should_lex_addsub() {
    let mut lexer = Lexer::new("9.10 + 2 - 10");
    lexer.lex();
    assert_eq!(
        lexer.tokens,
        [
            Token::Number(9.1),
            Token::Operator(Operator::Plus),
            Token::Number(2.0),
            Token::Operator(Operator::Minus),
            Token::Number(10.0),
            Token::EndOfFile,
        ]
    );
}

#[test]
fn should_lex_muldivmod() {
    let mut lexer = Lexer::new("5 * 40 % 10 / 10");
    lexer.lex();
    assert_eq!(
        lexer.tokens,
        [
            Token::Number(5.0),
            Token::Operator(Operator::Mul),
            Token::Number(40.0),
            Token::Operator(Operator::Modulus),
            Token::Number(10.0),
            Token::Operator(Operator::Div),
            Token::Number(10.0),
            Token::EndOfFile,
        ]
    );   
}

#[test]
fn should_lex_paren() {
    let mut lexer = Lexer::new("5 * (2 + 5)");
    lexer.lex();
    assert_eq!(
        lexer.tokens,
        [
            Token::Number(5.0),
            Token::Operator(Operator::Mul),
            Token::LParen,
            Token::Number(2.0),
            Token::Operator(Operator::Plus),
            Token::Number(5.0),
            Token::RParen,
            Token::EndOfFile,
        ]
    );
}

#[test]
fn should_lex_exponent() {
    let mut lexer = Lexer::new("5 ** (2 + 5)");
    lexer.lex();
    assert_eq!(
        lexer.tokens,
        [
            Token::Number(5.0),
            Token::Operator(Operator::Exponent),
            Token::LParen,
            Token::Number(2.0),
            Token::Operator(Operator::Plus),
            Token::Number(5.0),
            Token::RParen,
            Token::EndOfFile,
        ]
    );
}