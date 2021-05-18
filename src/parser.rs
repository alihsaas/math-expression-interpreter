use crate::ast::*;
use crate::token::*;

use crate::lexer::Lexer;

type PResult = Result<Node, String>;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(&text);
        lexer.lex();
        Self { lexer }
    }

    /*
        expr : addition-expression
        addition-expression : multiplication-expression ((PLUS|MINUS) multiplication-expression)*
        multiplication-expression : factor ((MUL|DIV|MODULUS) factor)*
        term : (PLUS | MINUS) factor | NUMBER | LPAREN expr RPAREN | EXPONENT
    */

    fn term(&mut self) -> PResult {
        let token = self.lexer.next();

        match token {
            Token::Number(_num) => Ok(Node::Token(token)),
            Token::Operator(Operator::Plus) | Token::Operator(Operator::Minus) => {
                Ok(Node::UnaryOperator(Box::new(UnaryOperator {
                    operator: token,
                    expression: self.term()?,
                })))
            }
            Token::LParen => {
                let result = self.addition_expr();
                let current_token = self.lexer.next();

                match current_token {
                    Token::RParen => result,
                    _ => Err(format!("Expected closing ')', got {}", current_token)),
                }
            }
            _ => Err(format!("Expected number got {:?}", token)),
        }
    }

    fn exponent_expr(&mut self) -> PResult {
        let mut node = self.term()?;

        while is_exponent(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Exponent) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.term()?,
                    }))
                }
                _ => return Err(format!("Expected '**', got {}", token)),
            }
        }

        Ok(node)
    }

    fn addition_expr(&mut self) -> PResult {
        let mut node = self.multiplication_expr()?;

        while is_addsub(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Plus) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.multiplication_expr()?,
                    }))
                }
                Token::Operator(Operator::Minus) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.multiplication_expr()?,
                    }))
                }
                _ => return Err(format!("Expected '*' or '/', got {}", token)),
            }
        }

        Ok(node)
    }

    fn multiplication_expr(&mut self) -> PResult {
        let mut node = self.exponent_expr()?;

        while is_muldivmod(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Mul) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.exponent_expr()?,
                    }))
                }
                Token::Operator(Operator::Div) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.exponent_expr()?,
                    }))
                }
                Token::Operator(Operator::Modulus) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.exponent_expr()?,
                    }))
                }
                _ => return Err(format!("Expected '*' or '/', got {}", token)),
            }
        }

        Ok(node)
    }

    pub fn parse(&mut self) -> PResult {
        let result = self.addition_expr();
        let current_token = self.lexer.peek();

        match current_token {
            Token::EndOfFile => result,
            _ => Err(format!("Expected EOF, got {}", current_token)),
        }
    }
}
