use super::super::*;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int(i64),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Var(String),
    /// Value
    Val(Value),
    /// Lambda form with one variable binding and a body
    Lambda(String, Box<Expr>),
    /// Function application with arguments
    App(Box<Expr>, Box<Expr>),
    /// If expression
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    Set(String, Box<Expr>),

    Quote(Sexp),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(s) => write!(f, "{}", s),
            Value::Int(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Val(v) => write!(f, "{}", v),
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Lambda(var, exp) => write!(f, "(λ ({}) {})", var, exp),
            Expr::App(rator, rand) => write!(f, "({} {})", rator, rand),
            Expr::If(test, csq, alt) => write!(f, "(if {} {} {})", test, csq, alt.as_ref()
                    .unwrap_or(&Box::new(Expr::Val(Value::Str(String::from("void")))))),
            Expr::Set(var, exp) => write!(f, "(set! {} {})", var, exp),
            Expr::Quote(exp) => write!(f, "'{}", exp),
        }
    }
}