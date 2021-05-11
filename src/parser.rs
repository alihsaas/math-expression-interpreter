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

    fn skip_whitespace(&mut self) {
        while Token::Whitespace == self.lexer.peek() {
            self.lexer.next();
        }
    }

    /*
        expr : addition-expression
        addition-expression : multiplication-expression ((PLUS|MINUS) multiplication-expression)*
        multiplication-expression : factor ((MUL|DIV) factor)*
        term : (PLUS | MINUS) factor | NUMBER | LPAREN expr RPAREN
    */

    fn term(&mut self) -> PResult {
        self.skip_whitespace();

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

                self.skip_whitespace();

                let current_token = self.lexer.next();

                match current_token {
                    Token::RParen => result,
                    _ => Err(format!("Expected closing ')', got {}", current_token)),
                }
            }
            _ => Err(format!("Expected number got {:?}", token)),
        }
    }

    fn addition_expr(&mut self) -> PResult {
        let mut node = self.multiplication_expr()?;

        while is_addsub(self.lexer.peek()) || is_whitespace(self.lexer.peek()) {
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
                Token::Whitespace => self.skip_whitespace(),
                _ => return Err(format!("Expected '*' or '/', got {}", token)),
            }
        }

        Ok(node)
    }

    fn multiplication_expr(&mut self) -> PResult {
        let mut node = self.term()?;

        while is_muldiv(self.lexer.peek()) || is_whitespace(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Mul) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.term()?,
                    }))
                }
                Token::Operator(Operator::Div) => {
                    node = Node::BinOperator(Box::new(BinOperator {
                        left: node,
                        operator: token,
                        right: self.term()?,
                    }))
                }
                Token::Whitespace => self.skip_whitespace(),
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
