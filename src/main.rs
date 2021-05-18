use interpreter::{Interpreter, Value};
use std::io;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

fn main() {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "exit" {
            break;
        }

        let mut interpreter = Interpreter::new();

        match interpreter.interpret(&input) {
            Ok(result) => match result {
                Value::Number(num) => println!("{}", num),
                Value::NoValue => (),
            },
            Err(err) => eprintln!("ERROR: {}", err),
        };
    }
}
