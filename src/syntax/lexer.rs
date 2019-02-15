#![allow(dead_code)]
use super::error::{Error, ErrorKind};
use super::token::{Token, TokenKind};
use std::iter::Peekable;
use std::str;

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
    fn token(&self, kind: TokenKind) -> Result<Token, Error> {
        let sz = kind.size() as u32;
        Ok(Token {
            kind,
            pos: self.pos - sz,
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
            Some('\n') => {
                self.line += 1;
                self.pos = 0;
                Some('\n')
            }
            Some(ch) => {
                self.pos += 1;
                Some(ch)
            }
            None => None,
        }
    }

    /// TODO: Parse identifiers/keywords without allocation
    fn consume_while<F: Fn(char) -> bool>(&mut self, pred: F) -> String {
        let mut s = String::new();
        while let Some(&n) = self.peek() {
            if pred(n) {
                match self.consume() {
                    Some(ch) => s.push(ch),
                    None => break,
                }
            } else {
                break;
            }
        }
        s
    }

    /// TODO: Parse identifiers/keywords without allocation
    fn read_identifier(&mut self) -> Result<Token, Error> {
        let ident = self.consume_while(is_identifier_char);
        // Should never happen
        if ident.is_empty() {
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
            if self.consume().is_none() {
                return self.error(ErrorKind::EOF);
            }
            self.token(TokenKind::Literal(ret))
        } else {
            self.error(ErrorKind::EOF)
        }
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let s = self.consume_while(char::is_numeric);
        let i = s.parse::<i64>().expect("Verified numeric chars");
        // generate this one manually so we don't have to calculate int length
        Ok(Token {
            pos: self.pos - s.len() as u32,
            line: self.line,
            kind: TokenKind::Integer(i),
        })
    }

    fn advance(&mut self, token: TokenKind) -> Result<Token, Error> {
        match self.consume() {
            Some(_) => self.token(token),
            None => self.error(ErrorKind::EOF),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        // Eat whitespace at beginning of current input
        self.consume_while(char::is_whitespace);

        if let Some(&ch) = self.peek() {
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
                x if x.is_numeric() => self.read_number(),
                x if is_identifier_char(x) => self.read_identifier(),
                _ => self.error(ErrorKind::Invalid(ch)),
            }
        } else {
            Ok(Token {
                kind: TokenKind::EOF,
                pos: self.pos,
                line: self.line,
            })
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        loop {
            match self.next_token() {
                Err(e) => return Err(e),
                Ok(token) => match token.kind {
                    TokenKind::EOF => break,
                    _ => tokens.push(token),
                },
            }
        }
        Ok(tokens)
    }
}

fn is_identifier_char(ch: char) -> bool {
    let valid = "~!@#$%^&*-_+=|?.<>/";
    ch.is_alphanumeric() || valid.contains(ch)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_list() {
        let input = "(cons (cons 1 2))";
        let expected = vec![
            Token {
                line: 0,
                pos: 0,
                kind: TokenKind::LeftParen,
            },
            Token {
                line: 0,
                pos: 1,
                kind: TokenKind::Identifier("cons".to_string()),
            },
            Token {
                line: 0,
                pos: 6,
                kind: TokenKind::LeftParen,
            },
            Token {
                line: 0,
                pos: 7,
                kind: TokenKind::Identifier("cons".to_string()),
            },
            Token {
                line: 0,
                pos: 12,
                kind: TokenKind::Integer(1),
            },
            Token {
                line: 0,
                pos: 14,
                kind: TokenKind::Integer(2),
            },
            Token {
                line: 0,
                pos: 15,
                kind: TokenKind::RightParen,
            },
            Token {
                line: 0,
                pos: 16,
                kind: TokenKind::RightParen,
            },
        ];

        let lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(expected, tokens);
    }

    #[test]
    fn lex_keywords() {
        use TokenKind::*;

        let input = "lambda define let if '`,;";
        let lexer = Lexer::new(input);
        let tokens = lexer
            .lex()
            .unwrap()
            .into_iter()
            .map(|tok| tok.kind)
            .collect::<Vec<TokenKind>>();
        let expected = vec![Lambda, Define, Let, If, Quote, Quasiquote, Unquote];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn lex_unexpected_eof() {
        let input = "\"this is a string literal";
        let lexer = Lexer::new(input);
        let tokens = lexer.lex();
        let expected = Err(Error {
            kind: ErrorKind::EOF,
            pos: input.len() as u32,
            line: 0,
        });
        assert_eq!(expected, tokens);
    }
}
