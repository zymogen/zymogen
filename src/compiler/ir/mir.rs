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

    /// Let form with one variable binding and a body to mirror A-normal form
    Let(String, Box<Expr>, Box<Expr>),
    /// Lambda form required variables, optional rest arg and body
    Lambda(Vec<String>, Option<String>, Box<Expr>),
    /// Function application with arguments
    App(Box<Expr>, Vec<Expr>),
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
            Expr::Let(var, val, body) => write!(f, "(let (({} {})) {})", var, val, body),
            Expr::Lambda(var, None, body) => write!(f, "(λ ({}) {})", var.join(" "), body),
            Expr::Lambda(var, Some(rest), body) => {
                write!(f, "(λ ({} . {}) {})", var.join(" "), rest, body)
            }
            Expr::App(rator, rand) => write!(
                f,
                "({} {})",
                rator,
                rand.into_iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expr::If(test, csq, None) => write!(f, "(if {} {})", test, csq),
            Expr::If(test, csq, Some(alt)) => write!(f, "(if {} {} {})", test, csq, alt),
            Expr::Set(var, exp) => write!(f, "(set! {} {})", var, exp),
            Expr::Quote(exp) => write!(f, "'{}", exp),
        }
    }
}
