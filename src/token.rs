use super::exception::Position;
use std::{fmt::Debug, rc::Rc};

#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub position: Position,
}

impl Token {
    pub fn new(token: TokenType, file: Rc<String>, line: usize, column: (usize, usize)) -> Self {
        Self {
            token,
            position: Position::new(line, column, file),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n\t{:?} -> {}", self.token, self.position)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(u16),
    Register(u8),
    Command(Command),
    Label(String),
    MprocessorDirective(MprocessorDirective),
    Comment(String),
    Colon,
    Eof,
    Eol,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Command {
    JMPNE,
    JMP,
    ADD,
    CALL,
    RET,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_camel_case_types)]
pub enum MprocessorDirective {
    M_include,
    M_error,
    M_define,
    M_undef,
    M_ifdef,
    M_ifndef,
    M_else,
    M_endif,
}

impl Command {
    pub fn all() -> [Command; 5] {
        [
            Command::JMPNE,
            Command::JMP,
            Command::ADD,
            Command::CALL,
            Command::RET,
        ]
    }
}
