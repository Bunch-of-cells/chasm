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
    JMPEQ,
    JMP,
    ADD,
    CALL,
    RET,
    CLR,
    SET,
    RAND,
    DRAW,
    OR,
    AND,
    XOR,
    SUB,
    SUBFROM,
    SHR,
    SHL,
    LOAD,
    STORE,
    POINT,
    ADDPTR,
    SETPTRCHR,
    SETPTRDEC,
    GETDELAY,
    GETKEY,
    SETDELAY,
    SETSOUND,

    OFFJMP,
    JMPEQKEY,
    JMPNEKEY,
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
    pub fn all() -> [Command; 30] {
        [
            Command::JMPNE,
            Command::JMPEQ,
            Command::JMP,
            Command::ADD,
            Command::CALL,
            Command::RET,
            Command::CLR,
            Command::SET,
            Command::RAND,
            Command::DRAW,
            Command::OR,
            Command::AND,
            Command::XOR,
            Command::SUB,
            Command::SUBFROM,
            Command::SHR,
            Command::SHL,
            Command::LOAD,
            Command::STORE,
            Command::POINT,
            Command::ADDPTR,
            Command::SETPTRCHR,
            Command::SETPTRDEC,
            Command::OFFJMP,
            Command::GETDELAY,
            Command::GETKEY,
            Command::SETDELAY,
            Command::SETSOUND,
            Command::JMPEQKEY,
            Command::JMPNEKEY,
        ]
    }

    pub fn is_valid_chip8_instruction(&self, args: &[TokenType]) -> bool {
        matches!(
            (self, args),
            (
                Command::ADD | Command::JMPNE | Command::JMPEQ,
                [TokenType::Number(0..=0xFF), TokenType::Register(_)]
                    | [TokenType::Register(_), TokenType::Number(0..=0xFF)]
                    | [TokenType::Register(_), TokenType::Register(_)]
            ) | (Command::RET | Command::CLR, [])
                | (
                    Command::CALL | Command::JMP | Command::OFFJMP,
                    [TokenType::Label(_) | TokenType::Number(_)]
                )
                | (
                    Command::SET,
                    [
                        TokenType::Register(_),
                        TokenType::Number(0..=0xFF) | TokenType::Register(_)
                    ]
                )
                | (
                    Command::RAND,
                    [TokenType::Register(_), TokenType::Number(0..=0xFF)]
                )
                | (
                    Command::DRAW,
                    [
                        TokenType::Register(_),
                        TokenType::Register(_),
                        TokenType::Number(0..=0xF)
                    ]
                )
                | (
                    Command::OR
                        | Command::AND
                        | Command::XOR
                        | Command::SUB
                        | Command::SUBFROM
                        | Command::SHL
                        | Command::SHR,
                    [TokenType::Register(_), TokenType::Register(_)]
                )
                | (
                    Command::LOAD
                        | Command::STORE
                        | Command::ADDPTR
                        | Command::SETPTRCHR
                        | Command::SETPTRDEC
                        | Command::GETDELAY
                        | Command::GETKEY
                        | Command::SETDELAY
                        | Command::JMPEQKEY
                        | Command::JMPNEKEY
                        | Command::SETSOUND,
                    [TokenType::Register(_)]
                )
                | (Command::POINT, [TokenType::Number(_) | TokenType::Label(_)])
        )
    }
}
