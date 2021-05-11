use crate::parser::Parser;
use crate::ast::*;
use crate::token::*;

#[derive(Debug)]
pub struct Interpreter {}

type IResult = Result<f64, String>;

impl Interpreter {

    pub fn new() -> Self {
        Self {}
    }

    fn visit(&mut self, node: &Node) -> IResult {
        match node {
            Node::BinOperator(node) => {
                match node.operator {
                    Token::Operator(Operator::Add) => Ok(self.visit(&node.left)? + self.visit(&node.right)?),
                    Token::Operator(Operator::Sub) => Ok(self.visit(&node.left)? - self.visit(&node.right)?),
                    Token::Operator(Operator::Mul) => Ok(self.visit(&node.left)? * self.visit(&node.right)?),
                    Token::Operator(Operator::Div) => Ok(self.visit(&node.left)? / self.visit(&node.right)?),
                    _ => Err(format!("Expected Operator, got {}.", node)),
                }
            },
            Node::Token(node) => {
                match node {
                    Token::Number(value) => Ok(*value),
                    _ => Err(format!("Expected Number, got {}", node)),
                }
            }
        }
    }

    pub fn interpret<'a>(&mut self, text: &'a str) -> IResult {
        let mut parser = Parser::new(text);
        self.visit(dbg!(&parser.parse()?))
    }    
}
