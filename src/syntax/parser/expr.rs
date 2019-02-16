#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Boolean(bool),
    Integer(i64),
    Identifier(String),
    Literal(String),
    List(Vec<Expression>),
    Keyword(Keyword),
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Keyword {
    Quote,
    Lambda,
    If,
    Set,
    Begin,
    Cond,
    And,
    Or,
    Case,
    Let,
    Letstar,
    Letrec,
    Do,
    Delay,
    Quasiquote,
    Else,
    Define,
    Unquote,
    UnquoteAt,
}
