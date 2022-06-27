use std::{fmt::Display, rc::Rc};

use crate::token::Token;

pub type Result<T> = core::result::Result<T, Exception>;

#[derive(Debug, Clone)]
pub struct Position {
    line: usize,
    column: (usize, usize),
    file: Rc<String>,
}

impl Position {
    pub fn new(line: usize, column: (usize, usize), file: Rc<String>) -> Self {
        Self { line, column, file }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}::{}::{}:{}",
            self.file, self.line, self.column.0, self.column.1
        )
    }
}

#[derive(Debug)]
pub struct Exception {
    position: Position,
    exception: ExceptionType,
    details: String,
}

impl Exception {
    pub fn new(
        exception: ExceptionType,
        file: Rc<String>,
        line: usize,
        column: (usize, usize),
        details: String,
    ) -> Exception {
        Self {
            position: Position { line, column, file },
            exception,
            details,
        }
    }

    pub fn from_token(exception: ExceptionType, token: &Token, details: String) -> Self {
        Self::new(
            exception,
            Rc::clone(&token.position.file),
            token.position.line,
            token.position.column,
            details,
        )
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Compiler Exception: {:?} :: file: {} :: line: {} :: column: {} to {} :: {}",
            self.exception,
            self.position.file,
            self.position.line,
            self.position.column.0,
            self.position.column.1,
            self.details,
        )
    }
}

#[derive(Debug)]
pub enum ExceptionType {
    NumberOverflow,
    InvalidToken,
    UnknownException,
    SyntaxError,
    // TooMuchRecursion,
    FileException,
    MprocessorException,
}
