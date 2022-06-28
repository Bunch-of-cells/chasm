use std::{fs, rc::Rc};

use intruction::convert_to_chip8;

fn main() {
    match lexer::lex(
        fs::read_to_string("example.chasm").unwrap().as_str(),
        Rc::new(String::from("example.chasm")),
    ) {
        Ok(tokens) => match parser::Parser::new(tokens).parse() {
            Ok(instructions) => {
                println!("{:?}", instructions);
                convert_to_chip8(instructions)
                    .iter()
                    .for_each(|i| print!("{i:0>4X} "));
                println!();
            }
            Err(err) => println!("{}", err),
        },
        Err(err) => println!("{}", err),
    }
}

mod exception;
mod intruction;
mod lexer;
mod parser;
mod token;

#[macro_export]
macro_rules! lex {
    ($($code: tt)*) => {
        crate::lexer::lex(stringify!($($code)*), std::rc::Rc::new(file!().to_string()))
    }
}
