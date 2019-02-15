#[derive(PartialEq, PartialOrd, Debug)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    Quote,
    Quasiquote,
    Unquote,
    Lambda,
    Let,
    Define,
    If,
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
    fn size(&self) -> usize {
        match self {
            TokenKind::Boolean(true) => 4,
            TokenKind::Boolean(false) => 5,
            TokenKind::Integer(i) => (i / 10) as usize,
            TokenKind::Literal(s) => s.len(),
            TokenKind::Identifier(s) => s.len(),
            _ => 1,
        }
    }
}

impl Token {
    fn kind(&self) -> &TokenKind {
        &self.kind
    }
}