use core::fmt;
use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub(crate) enum Token {
    TokenString(String),
    TokenNum(u64),
    TokenComma(char),
    TokenColon(char),
    TokenOpenSquareBracket(char),
    TokenClosedSquareBracket(char),
    TokenOpenCurlyBracket(char),
    TokenClosedCurlyBracket(char),
    TokenFalse(bool),
    TokenTrue(bool),
    TokenNull(String),
}

#[derive(Debug, Clone)]
pub(crate) struct LexerError {
    pub(crate) message: String,
}

impl LexerError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn get_string(_: char, stream: &mut Peekable<Chars>) -> String {
    let mut result = String::new();

    while let Some(c) = stream.next() {
        if c == '"' {
            break;
        }
        result.push(c);
    }

    result
}

fn get_number(c: char, it: &mut Peekable<Chars>) -> u64 {
    /*
    This currently only parses integers, it should be able to handle floating point numbers as well
    */
    let mut number = c.to_string().parse::<u64>().expect("Number parsing error");

    while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<u64>()) {
        number = number * 10 + digit;
        it.next();
    }

    number
}

fn parse_literal(
    candidates: String,
    stream: &mut Peekable<Chars>,
    rtype: Token,
) -> Result<Token, LexerError> {
    let mut it = candidates.chars();

    while let Some(&n) = stream.peek() {
        if let Some(candidate) = it.next() {
            if n != candidate {
                return Err(LexerError::new(String::from(
                    "Not matching any candidate for literal value",
                )));
            }
            stream.next();
        } else {
            return Ok(rtype);
        }
    }

    return Ok(rtype);
}

fn get_literal(c: char, it: &mut Peekable<Chars>) -> Result<Token, LexerError> {
    match c {
        'f' => {
            return parse_literal(String::from("alse"), it, Token::TokenFalse(false));
        }
        't' => {
            return parse_literal(String::from("rue"), it, Token::TokenTrue(true));
        }
        'n' => {
            return parse_literal(
                String::from("ull"),
                it,
                Token::TokenNull(String::from("null")),
            );
        }
        _ => {
            return Err(LexerError::new(String::from(format!(
                "No literal value starting with {} exists",
                c
            ))));
        }
    }
}

pub(crate) fn json_tokenize(text: String) -> Result<Vec<Token>, LexerError> {
    let mut it = text.chars().peekable();
    let mut result = Vec::<Token>::new();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                let n = get_number(c, &mut it);
                result.push(Token::TokenNum(n));
            }
            '"' => {
                it.next();
                let s = get_string(c, &mut it);
                result.push(Token::TokenString(s));
            }
            ',' => {
                it.next();
                result.push(Token::TokenComma(c));
            }
            '[' => {
                it.next();
                result.push(Token::TokenOpenSquareBracket(c));
            }
            ']' => {
                it.next();
                result.push(Token::TokenClosedSquareBracket(c));
            }
            '{' => {
                it.next();
                result.push(Token::TokenOpenCurlyBracket(c));
            }
            '}' => {
                it.next();
                result.push(Token::TokenClosedCurlyBracket(c));
            }
            ':' => {
                it.next();
                result.push(Token::TokenColon(c));
            }
            ' ' => {
                // Discarding the spaces for now
                it.next();
            }
            '\n' => {
                // Discarding the newlines for now
                it.next();
            }
            _ => {
                // Default behavior is that it might a literal
                it.next();

                match get_literal(c, &mut it) {
                    Ok(literal) => {
                        result.push(literal);
                    }
                    Err(error) => {
                        return Err(error);
                    }
                }
            }
        }
    }

    Ok(result)
}
