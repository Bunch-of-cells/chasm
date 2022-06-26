use std::{fmt::Display, rc::Rc};

#[derive(Debug)]
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
    // FileNotFound,
    // PreprocessorException,
}
