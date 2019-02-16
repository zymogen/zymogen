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

impl Expression {
    fn as_ident(&self) -> Option<String> {
        match self {
            Expression::Identifier(s) => Some(s.clone()),
            _ => None,
        }
    }

    fn unpack1(&self) -> Option<&Expression> {
        match self {
            Expression::List(v) => Some(v.get(0)?),
            _ => None,
        }
    }

    fn unpack2(&self) -> Option<(&Expression, &Expression)> {
        match self {
            Expression::List(v) => Some((v.get(0)?, v.get(1)?)),
            _ => None,
        }
    }

    fn unpack3(&self) -> Option<(&Expression, &Expression, &Expression)> {
        match self {
            Expression::List(v) => Some((v.get(0)?, v.get(1)?, v.get(2)?)),
            _ => None,
        }
    }
}
