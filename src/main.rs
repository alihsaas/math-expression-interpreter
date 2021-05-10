use crate::interpreter::Interpreter;
use std::io;

mod interpreter;
mod lexer;
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

        let mut interpreter = Interpreter::new(&input);

        match interpreter.expr() {
            Ok(result) => println!("{}", result),
            Err(err) => eprintln!("ERROR: {}", err),
        };
    }
}
