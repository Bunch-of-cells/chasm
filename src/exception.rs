use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

pub type Result<T> = core::result::Result<T, Box<dyn Exception>>;

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: (usize, usize),
    pub file: Rc<String>,
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

pub trait Exception {
    fn error(&self) -> &'static str;
    fn details(&self) -> &str;
    fn file(&self) -> Option<&str>;
    fn position(&self) -> Option<(usize, usize, usize)>; // line, col start, col end
}

impl Display for dyn Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compiler Exception: {} :: ", self.error())?;
        if let Some(file) = self.file() {
            write!(f, "file: {} :: ", file)?;
        }
        if let Some((line, col_start, col_end)) = self.position() {
            write!(
                f,
                "line: {} :: column: {} to {} :: ",
                line, col_start, col_end
            )?;
        }
        write!(f, "{}", self.details())
    }
}

pub struct NumberOverflow(pub String, pub Position);
impl Exception for NumberOverflow {
    fn error(&self) -> &'static str {
        "Number Overflow"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct InvalidToken(pub String, pub Position);
impl Exception for InvalidToken {
    fn error(&self) -> &'static str {
        "Invalid Token"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct UnknownException(pub String, pub Position);
impl Exception for UnknownException {
    fn error(&self) -> &'static str {
        "Unknown Exception"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct SyntaxError(pub String, pub Position);
impl Exception for SyntaxError {
    fn error(&self) -> &'static str {
        "Syntax Error"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct FileException(pub String, pub Position);
impl Exception for FileException {
    fn error(&self) -> &'static str {
        "File Exception"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct MprocessorException(pub String, pub Position);
impl Exception for MprocessorException {
    fn error(&self) -> &'static str {
        "Mprocessor Exception"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct NoMain(pub Rc<String>);
impl Exception for NoMain {
    fn error(&self) -> &'static str {
        "No Main"
    }

    fn details(&self) -> &str {
        "No Main function was found"
    }

    fn file(&self) -> Option<&str> {
        Some(self.0.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        None
    }
}

pub struct Undefined(pub String, pub Position);
impl Exception for Undefined {
    fn error(&self) -> &'static str {
        "Undefined Label"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}

pub struct Redefinition(pub String, pub Position);
impl Exception for Redefinition {
    fn error(&self) -> &'static str {
        "Redefinition"
    }

    fn details(&self) -> &str {
        &self.0
    }

    fn file(&self) -> Option<&str> {
        Some(self.1.file.as_str())
    }

    fn position(&self) -> Option<(usize, usize, usize)> {
        Some((self.1.line, self.1.column.0, self.1.column.1))
    }
}
