use super::exception::Position;
use std::{fmt::Debug, rc::Rc};

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

#[derive(Debug)]
pub enum TokenType {
    Number(u16),
    Register(u8),
    Command(Keyword),
    Label(String),
    PreprocessorDirective(PreprocessorDirective),
    Comment(String),
    Colon,
    Eof,
    Eol,
}

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Keyword {
    JMPNE,
    JMP,
    ADD,
    CALL,
    RET,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PreprocessorDirective {
    P_include,
    P_error,
    P_define,
    P_undef,
    P_ifdef,
    P_ifndef,
    P_else,
    P_endif,
}

impl Keyword {
    pub fn all() -> [Keyword; 5] {
        [
            Keyword::JMPNE,
            Keyword::JMP,
            Keyword::ADD,
            Keyword::CALL,
            Keyword::RET,
        ]
    }
}
