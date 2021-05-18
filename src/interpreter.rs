use crate::ast::*;
use crate::parser::Parser;
use crate::token::*;

#[derive(Debug)]
pub struct Interpreter {}

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Number(f64),
    NoValue,
}

type IResult = Result<Value, String>;

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn number_operation(&mut self, node: &BinOperator, callback: fn(f64, f64) -> f64) -> IResult {
        match (self.visit(&node.left)?, self.visit(&node.right)?) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(callback(a, b))),
            _ => Err(format!(
                "Expected Number for binary {:?}, got {:?}, {:?}",
                node.operator, node.left, node.right
            )),
        }
    }

    fn visit_bin_operator(&mut self, node: &BinOperator) -> IResult {
        match node.operator {
            Token::Operator(Operator::Plus) => self.number_operation(node, |a, b| a + b),
            Token::Operator(Operator::Minus) => self.number_operation(node, |a, b| a - b),
            Token::Operator(Operator::Mul) => self.number_operation(node, |a, b| a * b),
            Token::Operator(Operator::Div) => self.number_operation(node, |a, b| a / b),
            Token::Operator(Operator::Modulus) => self.number_operation(node, |a, b| a % b),
            Token::Operator(Operator::Exponent) => self.number_operation(node, |a, b| a.powf(b)),
            _ => Err(format!("Expected Operator, got {}.", node)),
        }
    }

    fn visit_unary_operator(&mut self, node: &UnaryOperator) -> IResult {
        match node.operator {
            Token::Operator(Operator::Plus) => self.visit(&node.expression),
            Token::Operator(Operator::Minus) => match self.visit(&node.expression)? {
                Value::Number(num) => Ok(Value::Number(-num)),
                other => Err(format!(
                    "Expected Number for Unary {:?}, got {:?}",
                    node.operator, other
                )),
            },
            _ => Err(format!(
                "Expected Unary Operator '+' or '-', got {}",
                node.operator
            )),
        }
    }

    fn visit_token(&mut self, node: &Token) -> IResult {
        match node {
            Token::Number(value) => Ok(Value::Number(*value)),
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
