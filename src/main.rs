use std::{fs, rc::Rc};

mod exception;
mod intruction;
mod lexer;
mod parser;
mod token;

use exception::Exception;
use intruction::convert_to_chip8;

fn main() {
    match run() {
        Ok(v) => {
            v.iter().for_each(|i| print!("{i:0>4X}"));
            println!();
            fs::write(
                "ibm.ch8",
                v.iter()
                    .flat_map(|i| [(i >> 8) as u8, (i & 0xFF) as u8].into_iter())
                    .collect::<Vec<_>>(),
            )
            .unwrap();
        }
        Err(e) => println!("{}", e),
    }
}

fn run() -> Result<Vec<u16>, Box<dyn Exception>> {
    let tokens = lexer::lex(
        fs::read_to_string("example.chasm").unwrap().as_str(),
        Rc::new(String::from("example.chasm")),
    )?;
    Ok(convert_to_chip8(parser::Parser::new(tokens).parse()?))
}
