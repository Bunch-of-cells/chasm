use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs,
    rc::Rc,
    vec,
};

use crate::{
    exception::{FileException, NoMain, Result, SyntaxError, Undefined},
    intruction::InstructionArg,
    lexer::lex,
    token::{Command, MprocessorDirective, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
    ifs: Vec<bool>,
    defined: HashSet<String>,
    labels: HashMap<String, usize>,
    instructions: Vec<(Command, Vec<TokenType>)>,
    has_main: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: 0,
            ifs: Vec::new(),
            defined: HashSet::new(),
            labels: HashMap::new(),
            instructions: Vec::new(),
            has_main: false,
        }
    }

    fn advance(&mut self) {
        self.current_token += 1;
    }

    fn ignore(&self) -> bool {
        self.ifs.iter().any(|&b| !b)
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current_token]
    }

    fn statements(&mut self, func: fn(&mut Self) -> Result<()>) -> Result<()> {
        while self.current_token().token != TokenType::Eof {
            func(self)?;
        }
        if !self.ifs.is_empty() {
            return Err(Box::new(SyntaxError(
                "%?# or %?! were not ended".to_string(),
                self.current_token().position.clone(),
            )));
        }
        if !self.has_main {
            return Err(Box::new(NoMain(self.current_token().position.file.clone())));
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Vec<(Command, Vec<InstructionArg>)>> {
        self.statements(Self::labels)?;
        self.current_token = 0;
        self.statements(Self::statement)?;
        Ok(self.convert_instructions())
    }

    fn convert_instructions(&self) -> Vec<(Command, Vec<InstructionArg>)> {
        let mut instructions = vec![
            (
                Command::CALL,
                vec![InstructionArg::Label(self.labels["main"] as u16 + 2)],
            ),
            (Command::JMP, vec![InstructionArg::Label(1)]),
        ];
        for (cmd, args) in &self.instructions {
            let mut new_args = Vec::new();
            for arg in args {
                match arg {
                    TokenType::Register(r) => new_args.push(InstructionArg::Reg(*r)),
                    TokenType::Number(n) if *cmd == Command::CHIP => {
                        new_args.push(InstructionArg::Chip8(*n))
                    }
                    TokenType::Number(n) => new_args.push(InstructionArg::Num(*n)),
                    TokenType::Label(l) => {
                        new_args.push(InstructionArg::Label(self.labels[l] as u16 + 2))
                    }
                    _ => unreachable!(),
                }
            }
            instructions.push((cmd.clone(), new_args));
        }
        instructions
    }

    fn labels(&mut self) -> Result<()> {
        let ignore = self.ignore();
        match self.current_token().token {
            TokenType::Label(_) if !ignore => return self.label(),
            TokenType::MprocessorDirective(_) => return self.m_process(),
            TokenType::Number(_) => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put numbers anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Register(_) => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put registers anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Command(_) => {
                while !matches!(
                    self.current_token().token,
                    TokenType::Comment(_) | TokenType::Eol | TokenType::Eof
                ) {
                    self.advance();
                }
                self.current_token -= 1;
            }
            TokenType::Comment(_) => (),
            TokenType::Colon => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put colons anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Eof => unreachable!(),
            TokenType::Eol => (),
            _ => (),
        }
        self.advance();
        Ok(())
    }

    fn statement(&mut self) -> Result<()> {
        let ignore = self.ignore();
        match self.current_token().token {
            TokenType::Number(_) if !ignore => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put numbers anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Register(_) if !ignore => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put registers anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Command(_) if !ignore => return self.command(),
            TokenType::Label(ref l) if !ignore => {
                let l = l.clone();
                if let Some(k) = self.labels.get_mut(&l) {
                    *k = self.instructions.len();
                }
                self.advance(); // colon
            }
            TokenType::MprocessorDirective(_) => return self.m_process(),
            TokenType::Comment(_) => (),
            TokenType::Colon if !ignore => {
                return Err(Box::new(SyntaxError(
                    "You cant just randomly put colons anywhere you like you know".to_string(),
                    self.current_token().position.clone(),
                )))
            }
            TokenType::Eof => unreachable!(),
            TokenType::Eol => (),
            _ => (),
        }
        self.advance();
        Ok(())
    }

    fn m_process(&mut self) -> Result<()> {
        let ignore = self.ignore();
        let dir = self.current_token().clone();
        self.advance();
        let arg = if let TokenType::Comment(ref c) = self.current_token().token {
            c.trim()
        } else {
            return Err(Box::new(SyntaxError(
                "Expected ';' after a preprocessor directive".to_string(),
                self.current_token().position.clone(),
            )));
        };

        if let TokenType::MprocessorDirective(ref p) = dir.token {
            match p {
                MprocessorDirective::M_include if !ignore => match fs::read_to_string(arg) {
                    Ok(code) => {
                        let tokens = lex(&code, Rc::new(arg.to_string()))?;
                        self.tokens
                            .splice(self.current_token - 1..=self.current_token, tokens);
                        self.current_token -= 1;
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Box::new(FileException(
                            format!("Could not read file: {}", e),
                            self.current_token().position.clone(),
                        )))
                    }
                },
                MprocessorDirective::M_error if !ignore => {
                    return Err(Box::new(SyntaxError(
                        arg.to_string(),
                        self.current_token().position.clone(),
                    )))
                }
                MprocessorDirective::M_define if !ignore => {
                    let arg = arg.to_string();
                    self.defined.insert(arg);
                    self.tokens
                        .drain(self.current_token - 1..=self.current_token);
                    self.current_token -= 1;
                    return Ok(());
                }
                MprocessorDirective::M_undef if !ignore => {
                    let arg = arg.to_string();
                    if !self.defined.remove(&arg) {
                        return Err(Box::new(Undefined(
                            format!("Undefined flag: {}", arg),
                            self.current_token().position.clone(),
                        )));
                    }
                    self.tokens
                        .drain(self.current_token - 1..=self.current_token);
                    self.current_token -= 1;
                    return Ok(());
                }
                MprocessorDirective::M_ifdef => {
                    let b = self.defined.contains(arg);
                    self.ifs.push(b);
                }
                MprocessorDirective::M_ifndef => {
                    let b = !self.defined.contains(arg);
                    self.ifs.push(b);
                }
                MprocessorDirective::M_else => {
                    if let Some(c) = self.ifs.last_mut() {
                        *c = !*c;
                    } else {
                        return Err(Box::new(SyntaxError(
                            "%?| without any %?# or %?!".to_string(),
                            self.current_token().position.clone(),
                        )));
                    }
                }
                MprocessorDirective::M_endif => {
                    if self.ifs.pop().is_none() {
                        return Err(Box::new(SyntaxError(
                            "%?- without any %?# or %?!".to_string(),
                            self.current_token().position.clone(),
                        )));
                    }
                }
                _ => (),
            }
        }
        self.advance();
        Ok(())
    }

    fn command(&mut self) -> Result<()> {
        let command = self.current_token().clone();
        self.advance();
        let mut args = vec![];
        while !matches!(
            self.current_token().token,
            TokenType::Comment(_) | TokenType::Eol | TokenType::Eof
        ) {
            if !matches!(
                self.current_token().token,
                TokenType::Register(_) | TokenType::Label(_) | TokenType::Number(_)
            ) {
                return Err(Box::new(SyntaxError(
                    "Expected a register, label, or number as an argument".to_string(),
                    self.current_token().position.clone(),
                )));
            }
            if let TokenType::Label(ref l) = self.current_token().token {
                if !self.labels.contains_key(l) {
                    return Err(Box::new(Undefined(
                        format!("label '{}' is not defined anywhere", l),
                        self.current_token().position.clone(),
                    )));
                }
            }
            args.push(self.current_token().token.clone());
            self.advance();
        }
        if let TokenType::Command(c) = command.token {
            if !c.is_valid_chip8_instruction(&args) {
                return Err(Box::new(SyntaxError(
                    format!("Invalid arguments to command {:?}", c),
                    command.position.clone(),
                )));
            }
            self.instructions.push((c, args));
        }
        Ok(())
    }

    fn label(&mut self) -> Result<()> {
        let label = if let TokenType::Label(ref l) = self.current_token().token {
            l.clone()
        } else {
            unreachable!()
        };

        if self.tokens[self.current_token + 1].token != TokenType::Colon {
            return Err(Box::new(SyntaxError(
                "Expected : after label name".to_string(),
                self.current_token().position.clone(),
            )));
        }

        match self.labels.entry(label) {
            Entry::Vacant(e) => {
                if e.key() == "main" {
                    self.has_main = true;
                }
                e.insert(0);
            }
            Entry::Occupied(e) => {
                return Err(Box::new(SyntaxError(
                    format!(
                        "Label '{}' has already been defined and cannot be redefined",
                        e.key()
                    ),
                    self.current_token().position.clone(),
                )));
            }
        }

        self.advance();
        self.advance();
        Ok(())
    }
}
