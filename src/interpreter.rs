use crate::ast::*;
use crate::parser::Parser;
use crate::token::*;

#[derive(Debug)]
pub struct Interpreter {}

type IResult = Result<f64, String>;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn visit_bin_operator(&mut self, node: &BinOperator) -> IResult {
        match node.operator {
            Token::Operator(Operator::Plus) => {
                Ok(self.visit(&node.left)? + self.visit(&node.right)?)
            }
            Token::Operator(Operator::Minus) => {
                Ok(self.visit(&node.left)? - self.visit(&node.right)?)
            }
            Token::Operator(Operator::Mul) => {
                Ok(self.visit(&node.left)? * self.visit(&node.right)?)
            }
            Token::Operator(Operator::Div) => {
                Ok(self.visit(&node.left)? / self.visit(&node.right)?)
            }
            _ => Err(format!("Expected Operator, got {}.", node)),
        }
    }

    fn visit_unary_operator(&mut self, node: &UnaryOperator) -> IResult {
        match node.operator {
            Token::Operator(Operator::Plus) => self.visit(&node.expression),
            Token::Operator(Operator::Minus) => Ok(self.visit(&node.expression)? * -1f64),
            _ => Err(format!(
                "Expected Unary Operator '+' or '-', got {}",
                node.operator
            )),
        }
    }

    fn visit_token(&mut self, node: &Token) -> IResult {
        match node {
            Token::Number(value) => Ok(*value),
            _ => Err(format!("Expected Number, got {}", node)),
        }
    }

    fn visit(&mut self, node: &Node) -> IResult {
        match node {
            Node::BinOperator(node) => self.visit_bin_operator(node),
            Node::UnaryOperator(node) => self.visit_unary_operator(node),
            Node::Token(node) => self.visit_token(node),
        }
    }

    pub fn interpret(&mut self, text: &str) -> IResult {
        let mut parser = Parser::new(text);
        self.visit(dbg!(&parser.parse()?))
    }
}
