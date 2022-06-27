use std::{collections::HashSet, fs, mem, rc::Rc, vec};

use crate::{
    exception::{Exception, ExceptionType, Result},
    lexer::lex,
    token::{MprocessorDirective, Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current_token: usize,
    ifs: Vec<bool>,
    defined: HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current_token: 0,
            ifs: Vec::new(),
            defined: HashSet::new(),
            // labels: vec![],
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

    fn statements(&mut self) -> Result<()> {
        while self.current_token().token != TokenType::Eof {
            self.statement()?;
        }
        if !self.ifs.is_empty() {
            return Err(Exception::from_token(
                ExceptionType::SyntaxError,
                self.current_token(),
                "%?# or %?! were not ended".to_string(),
            ));
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<()> {
        self.statements()
    }

    fn statement(&mut self) -> Result<()> {
        let ignore = self.ignore();
        match self.current_token().token {
            TokenType::Number(_) if !ignore => {
                return Err(Exception::from_token(
                    ExceptionType::SyntaxError,
                    self.current_token(),
                    "You cant just randomly put numbers anywhere you like you know".to_string(),
                ))
            }
            TokenType::Register(_) if !ignore => {
                return Err(Exception::from_token(
                    ExceptionType::SyntaxError,
                    self.current_token(),
                    "You cant just randomly put registers anywhere you like you know".to_string(),
                ))
            }
            TokenType::Command(_) if !ignore => return self.command(),
            TokenType::Label(_) if !ignore => return self.label(),
            TokenType::MprocessorDirective(_) => return self.preprocess(),
            TokenType::Comment(_) => (),
            TokenType::Colon if !ignore => {
                return Err(Exception::from_token(
                    ExceptionType::SyntaxError,
                    self.current_token(),
                    "You cant just randomly put colons anywhere you like you know".to_string(),
                ))
            }
            TokenType::Eof => unreachable!(),
            TokenType::Eol => (),
            _ => (),
        }
        self.advance();
        Ok(())
    }

    fn preprocess(&mut self) -> Result<()> {
        let ignore = self.ignore();
        let dir = self.current_token().clone();
        self.advance();
        let arg = if let TokenType::Comment(ref c) = self.current_token().token {
            c.trim()
        } else {
            return Err(Exception::from_token(
                ExceptionType::SyntaxError,
                self.current_token(),
                "Expected ';' after a preprocessor directive".to_string(),
            ));
        };

        if let TokenType::MprocessorDirective(ref p) = dir.token {
            match p {
                MprocessorDirective::M_include if !ignore => match fs::read_to_string(arg) {
                    Ok(code) => {
                        let tokens = lex(&code, Rc::new(arg.to_string()))?;
                        let prev = mem::replace(&mut self.tokens, tokens);
                        let prev_token = mem::replace(&mut self.current_token, 0);
                        self.parse()?;
                        self.tokens = prev;
                        self.current_token = prev_token;
                    }
                    Err(e) => {
                        return Err(Exception::from_token(
                            ExceptionType::FileException,
                            self.current_token(),
                            format!("Could not read file: {}", e),
                        ));
                    }
                },
                MprocessorDirective::M_error if !ignore => {
                    return Err(Exception::from_token(
                        ExceptionType::MprocessorException,
                        self.current_token(),
                        arg.to_string(),
                    ));
                }
                MprocessorDirective::M_define if !ignore => {
                    let arg = arg.to_string();
                    self.defined.insert(arg);
                }
                MprocessorDirective::M_undef if !ignore => {
                    let arg = arg.to_string();
                    if !self.defined.remove(&arg) {
                        return Err(Exception::from_token(
                            ExceptionType::MprocessorException,
                            self.current_token(),
                            format!("Undefined flag: {}", arg),
                        ));
                    }
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
                        return Err(Exception::from_token(
                            ExceptionType::SyntaxError,
                            &dir,
                            "%?| without any %?# or %?!".to_string(),
                        ));
                    }
                }
                MprocessorDirective::M_endif => {
                    if self.ifs.pop().is_none() {
                        return Err(Exception::from_token(
                            ExceptionType::SyntaxError,
                            &dir,
                            "%?- without any %?# or %?!".to_string(),
                        ));
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
            args.push(self.current_token().token.clone());
            self.advance();
        }
        if let TokenType::Command(ref c) = command.token {
            if !c.is_valid_chip8_instruction(&args) {
                return Err(Exception::from_token(
                    ExceptionType::SyntaxError,
                    &command,
                    format!("Invalid arguments to command {:?}", c),
                ));
            }
        }
        self.advance();
        Ok(())
    }

    fn label(&mut self) -> Result<()> {
        self.advance();
        Ok(())
    }
}
