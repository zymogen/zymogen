use super::token::{Token, TokenKind};
use std::str;
use std::iter::Peekable;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum ErrorKind {
    EOF,
    Invalid(char),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Error {
    kind: ErrorKind,
    pos: u32,
    line: u32,
}


pub struct Lexer<'s> {
    input: Peekable<str::Chars<'s>>,
    pos: u32,
    line: u32,
}

impl<'s> Lexer<'s> {
    /// Construct a new Lexer that operates on a valid UTF-8 input &str
    pub fn new(input: &'s str) -> Lexer<'s> {
        Lexer {
            input: input.chars().peekable(),
            pos: 0,
            line: 0,
        }
    }

    /// Return a [`Token`] containing source position
    fn token(&self, kind: TokenKind) -> Result<Token, Error>  {
        Ok(Token {
            kind,
            pos: self.pos,
            line: self.line,
        })
    }

    /// Return an [`Error`] containing source position
    fn error(&self, kind: ErrorKind) -> Result<Token, Error> {
        Err(Error {
            kind,
            pos: self.pos,
            line: self.line,
        })
    }

    /// Peek at the next [`char`] in the source, if it exists
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    /// Consume the next [`char`] and advance internal source position
    fn consume(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') =>  {
                self.line += 1;
                self.pos = 0;
                Some('\n')
            },
            Some(ch) => {
                self.pos += 1;
                Some(ch)
            }
            None => None        
        }
    }

    /// TODO: Parse identifiers/keywords without allocation
    fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        let mut s = String::new();
        while let Some(&n) = self.peek() {
            match pred(n) {
                true => match self.consume() {
                    Some(ch) => s.push(ch),
                    None => break,
                },
                false => break,
            }
        }
        s
    }

    /// TODO: Parse identifiers/keywords without allocation
    fn read_identifier(&mut self) -> Result<Token, Error> {
        let ident = self.consume_while(is_identifier_char);
        if ident.len() == 0 {
            return self.error(ErrorKind::EOF);
        }
        match ident.as_ref() {
            "lambda" => self.token(TokenKind::Lambda),
            "if" => self.token(TokenKind::If),
            "let" => self.token(TokenKind::Let),
            "define" => self.token(TokenKind::Define),
            "true" => self.token(TokenKind::Boolean(true)),
            "false" => self.token(TokenKind::Boolean(false)),
            _ => self.token(TokenKind::Identifier(ident)),
        }
    }


    fn read_literal(&mut self) -> Result<Token, Error> {
        if let Some('"') = self.consume() {
            let ret = self.consume_while(|ch| ch != '"');
            self.consume().ok_or(self.error(ErrorKind::EOF))?;
            self.token(TokenKind::Literal(ret))
        } else {
            self.error(ErrorKind::EOF)
        }
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let s = self.consume_while(char::is_numeric);
        self.token(TokenKind::Integer(s.parse::<i64>().expect("Verified numeric chars")))
    }

    
    fn advance(&mut self, token: TokenKind) -> Result<Token, Error> {
        match self.consume() {
            Some(_) => self.token(token),
            None => self.error(ErrorKind::EOF)
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        // Eat whitespace at beginning of current input
        self.consume_while(char::is_whitespace);

        if let Some(ch) = self.peek() {
            match ch {
                '(' => self.advance(TokenKind::LeftParen),
                ')' => self.advance(TokenKind::RightParen),
                ';' => {
                    // Comment, read til end of line
                    self.consume_while(|ch| ch != '\n');
                    self.next_token()
                }
                '\'' => self.advance(TokenKind::Quote),
                '`' => self.advance(TokenKind::Quasiquote),
                ',' => self.advance(TokenKind::Unquote),
                '"' => self.read_literal(),
                x @ _ if x.is_numeric() => self.read_number(),
                x @ _ if is_identifier_char(*x) => self.read_identifier(),
                _ => self.error(ErrorKind::Invalid(*ch)),
            }
        } else {
            Ok(Token {
                kind: TokenKind::EOF,
                pos: self.pos,
                line: self.line,
            })
        }
    }

}

fn is_identifier_char(ch: char) -> bool {
    let valid = "~!@#$%^&*-_+=|?.<>/";
    ch.is_alphanumeric() || valid.contains(ch)
}
