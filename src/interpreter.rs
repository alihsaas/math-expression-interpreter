use crate::lexer::Lexer;
use crate::token::*;

#[derive(Debug)]
pub struct Interpreter<'a> {
    lexer: Lexer<'a>,
}

type IResult = Result<f64, String>;

impl<'a> Interpreter<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(text);
        lexer.lex();
        Self { lexer }
    }

    fn skip_whitespace(&mut self) {
        while Token::Whitespace == self.lexer.peek() {
            self.lexer.next();
        }
    }

    fn term(&mut self) -> IResult {
        self.skip_whitespace();
        let token = self.lexer.next();

        match token {
            Token::Number(num) => Ok(num),
            Token::Operator(Operator::Add) => self.term(),
            Token::Operator(Operator::Sub) => Ok(self.term()? * -1f64),
            Token::LParen => {
                let result = self.expr();

                let current_token = self.lexer.next();

                match current_token {
                    Token::RParen => result,
                    _ => Err(format!("Expected closing ')' got {}", current_token)),
                }
            }
            _ => Err(format!("Expected number got {:?}", token)),
        }
    }

    fn addition_expr(&mut self) -> IResult {
        let mut result = self.multiplication_expr()?;

        while is_addsub(self.lexer.peek()) || is_whitespace(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Add) => result += self.multiplication_expr()?,
                Token::Operator(Operator::Sub) => result -= self.multiplication_expr()?,
                Token::Whitespace => self.skip_whitespace(),
                _ => return Err(format!("Expected '+' or '-' got {}", token)),
            }
        }

        Ok(result)
    }

    fn multiplication_expr(&mut self) -> IResult {
        let mut result = self.term()?;

        while is_muldiv(self.lexer.peek()) || is_whitespace(self.lexer.peek()) {
            let token = self.lexer.next();
            match token {
                Token::Operator(Operator::Mul) => result *= self.term()?,
                Token::Operator(Operator::Div) => result /= self.term()?,
                Token::Whitespace => self.skip_whitespace(),
                _ => return Err(format!("Expected '*' or '/' got {}", token)),
            }
        }

        Ok(result)
    }

    pub fn expr(&mut self) -> IResult {
        self.addition_expr()
    }
}
