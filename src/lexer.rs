use crate::{
    exception::{Exception, ExceptionType},
    token::{Keyword, PreprocessorDirective, Token, TokenType},
};
use std::{num::IntErrorKind, rc::Rc};

const LITERALS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

pub fn lex(input: &str, filename: Rc<String>) -> Result<Vec<Token>, Exception> {
    println!("{}", input);
    let mut tokens = Vec::new();
    let mut chars = input.chars().enumerate().peekable();
    let mut line = 1;
    let mut last_line = 0;
    let mut last = 0;

    while let Some((j, mut c)) = chars.next() {
        c = c.to_ascii_uppercase();
        let i = j - last_line + 1;
        last = i + 1;
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                if c == '\n' {
                    line += 1;
                    last_line = j + 1;
                    tokens.push(Token::new(
                        TokenType::Eol,
                        Rc::clone(&filename),
                        line,
                        (i, i + 1),
                    ));
                }
            }
            ':' => tokens.push(Token::new(
                TokenType::Colon,
                Rc::clone(&filename),
                line,
                (i, i + 1),
            )),
            ';' => {
                let mut word = String::new();
                let start = i;
                let mut end = j + 2;
                for (j, c) in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                    end = j + 2;
                    word.push(c);
                }
                tokens.push(Token::new(
                    TokenType::Comment(word),
                    Rc::clone(&filename),
                    line,
                    (start, end - last_line),
                ));
                line += 1;
                last_line = end;
                tokens.push(Token::new(
                    TokenType::Eol,
                    Rc::clone(&filename),
                    line,
                    (end, end + 1),
                ));
            }
            '%' => match chars.next() {
                Some((_, c)) => match c {
                    '+' => tokens.push(Token::new(
                        TokenType::PreprocessorDirective(PreprocessorDirective::P_include),
                        Rc::clone(&filename),
                        line,
                        (i, i + 2),
                    )),
                    '#' => match chars.next() {
                        Some((_, c)) => match c {
                            '+' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_define),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            '-' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_undef),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            _ => {
                                return Err(Exception::new(
                                    ExceptionType::InvalidToken,
                                    Rc::clone(&filename),
                                    line,
                                    (i, i + 2),
                                    format!("Invalid preprocessor directive '#{}'", c),
                                ))
                            }
                        },
                        None => {
                            return Err(Exception::new(
                                ExceptionType::SyntaxError,
                                Rc::clone(&filename),
                                line,
                                (i, i + 1),
                                "Expected+ or - after %#".to_string(),
                            ))
                        }
                    },
                    '?' => match chars.next() {
                        Some((_, c)) => match c {
                            '#' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_ifdef),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            '!' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_ifndef),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            '|' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_else),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            '-' => tokens.push(Token::new(
                                TokenType::PreprocessorDirective(PreprocessorDirective::P_endif),
                                Rc::clone(&filename),
                                line,
                                (i, i + 3),
                            )),
                            _ => {
                                return Err(Exception::new(
                                    ExceptionType::InvalidToken,
                                    Rc::clone(&filename),
                                    line,
                                    (i, i + 2),
                                    format!("Invalid preprocessor directive '%{}'", c),
                                ))
                            }
                        },
                        None => {
                            return Err(Exception::new(
                                ExceptionType::SyntaxError,
                                Rc::clone(&filename),
                                line,
                                (i, i + 1),
                                "Expected #, !, | or - after %?".to_string(),
                            ))
                        }
                    },
                    '!' => tokens.push(Token::new(
                        TokenType::PreprocessorDirective(PreprocessorDirective::P_error),
                        Rc::clone(&filename),
                        line,
                        (i, i + 2),
                    )),
                    _ => {
                        return Err(Exception::new(
                            ExceptionType::SyntaxError,
                            Rc::clone(&filename),
                            line,
                            (i, i + 2),
                            format!("Invalid preprocessor directive '{}'", c),
                        ))
                    }
                },
                None => {
                    return Err(Exception::new(
                        ExceptionType::SyntaxError,
                        Rc::clone(&filename),
                        line,
                        (i, i + 1),
                        "Expected a preprocessor directive after %".to_string(),
                    ))
                }
            },
            _ if c.is_digit(10) => {
                let mut num = c.to_string();
                let start = i;
                let mut end = i + 1;
                let mut base = 10;
                if c == '0' && chars.peek().is_some() {
                    match chars.peek().unwrap().1 {
                        'b' | 'B' => {
                            base = 2;
                            chars.next();
                        }
                        'o' | 'O' => {
                            base = 8;
                            chars.next();
                        }
                        'x' | 'X' => {
                            base = 16;
                            chars.next();
                        }
                        _ => (),
                    }
                }
                while let Some((i, c)) = chars.peek() {
                    if !matches!(c, '0'..='9' | 'a'..='f' | 'A'..='F') {
                        break;
                    }
                    end = *i + 2;
                    num.push(*c);
                    chars.next();
                }

                tokens.push(Token::new(
                    TokenType::Number(match u16::from_str_radix(&num, base) {
                        Ok(num) if num > 0xFFF => {
                            return Err(Exception::new(
                                ExceptionType::NumberOverflow,
                                Rc::clone(&filename),
                                line,
                                (start, end),
                                format!("Number {} is too large", num),
                            ))
                        }
                        Ok(num) => num,
                        Err(err) => {
                            let (exception, details) = match err.kind() {
                                IntErrorKind::InvalidDigit => (
                                    ExceptionType::SyntaxError,
                                    format!("Invalid digit found while parsing number '{}'", num),
                                ),
                                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => (
                                    ExceptionType::NumberOverflow,
                                    format!("Number '{}' is too large", num),
                                ),
                                _ => (
                                    ExceptionType::UnknownException,
                                    format!(
                                        "An unknown excpetion occured while parsing number '{}'",
                                        num
                                    ),
                                ),
                            };
                            return Err(Exception::new(
                                exception,
                                Rc::clone(&filename),
                                line,
                                (start, end),
                                details,
                            ));
                        }
                    }),
                    Rc::clone(&filename),
                    line,
                    (start, end),
                ));
            }
            _ if LITERALS.contains(c) => {
                let mut word = c.to_string();
                let start = i;
                let mut end = i + 1;
                while let Some((i, c)) = chars.peek() {
                    if !(LITERALS.contains(*c) || c.is_numeric()) {
                        break;
                    }
                    end = *i + 2;
                    word.push(c.to_ascii_uppercase());
                    chars.next();
                }
                end -= last_line;
                if let Some(token) = None
                    .or_else(|| {
                        if let [b'V', x @ (b'0'..=b'9' | b'A'..=b'F')] = word.as_bytes() {
                            Some(TokenType::Register(match x {
                                b'0'..=b'9' => x - b'0',
                                _ => x - b'A' + 10,
                            }))
                        } else {
                            None
                        }
                    })
                    .or_else(|| {
                        Keyword::all()
                            .into_iter()
                            .find(|cmd| word == format!("{:?}", cmd))
                            .map(TokenType::Command)
                    })
                    .or_else(|| {
                        if word.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                            Some(TokenType::Label(word.to_ascii_lowercase()))
                        } else {
                            None
                        }
                    })
                {
                    tokens.push(Token::new(token, Rc::clone(&filename), line, (start, end)));
                } else {
                    return Err(Exception::new(
                        ExceptionType::InvalidToken,
                        Rc::clone(&filename),
                        line,
                        (start, end),
                        format!("Invalid token found while parsing '{}'", word),
                    ));
                }
            }
            _ => {
                return Err(Exception::new(
                    ExceptionType::InvalidToken,
                    Rc::clone(&filename),
                    line,
                    (i, i + 1),
                    format!("Invalid token found while parsing '{}'", c),
                ));
            }
        }
    }

    if let Some(s) = last.checked_sub(last_line) {
        last = s;
    }

    tokens.push(Token::new(
        TokenType::Eof,
        Rc::clone(&filename),
        line,
        (last, 0),
    ));
    Ok(tokens)
}
