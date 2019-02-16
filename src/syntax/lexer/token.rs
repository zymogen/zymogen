#![allow(dead_code)]

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Quote,
    Quasiquote,
    Unquote,
    UnquoteAt,
    Dot,
    Boolean(bool),
    Integer(i64),
    Literal(String),
    Identifier(String),
    EOF,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub pos: u32,
}

impl TokenKind {
    pub fn size(&self) -> usize {
        match self {
            TokenKind::Boolean(true) => 4,
            TokenKind::Boolean(false) => 5,
            TokenKind::Integer(i) => i.to_string().len(),
            TokenKind::Literal(s) => s.len(),
            TokenKind::Identifier(s) => s.len(),
            _ => 1,
        }
    }
}
