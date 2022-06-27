use std::{fs, rc::Rc};

fn main() {
    match lexer::lex(
        fs::read_to_string("example.chasm").unwrap().as_str(),
        Rc::new(String::from("example.chasm")),
    ) {
        Ok(tokens) => match parser::Parser::new(tokens).parse() {
            Ok(ast) => println!("{:?}", ast),
            Err(err) => println!("{}", err),
        },
        Err(err) => println!("{}", err),
    }
}

mod exception;
mod lexer;
mod node;
mod parser;
mod token;

#[macro_export]
macro_rules! lex {
    ($($code: tt)*) => {
        crate::lexer::lex(stringify!($($code)*), std::rc::Rc::new(file!().to_string()))
    }
}
