mod error;
mod lexer;
mod parser;

pub use error::{Error, ErrorKind};
pub use lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub use parser::{Expression, Keyword, Parser};

pub fn lex<S: AsRef<str>>(s: S) -> Result<Vec<Token>, String> {
    match Lexer::new(s.as_ref()).lex() {
        Ok(v) => Ok(v),
        Err(e) => Err(e.message(s)),
    }
}

pub fn parse<S: AsRef<str>>(s: S) -> Result<Vec<Expression>, String> {
    match Parser::new(s.as_ref()).parse() {
        Ok(v) => Ok(v),
        Err(e) => Err(e.message(s)),
    }
}
