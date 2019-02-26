pub mod hir;
pub mod mir;
pub mod bytecode;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Str(String),
    Bool(bool),
    Int(i64),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "{}", s),
            Value::Bool(s) => write!(f, "{}", s),
            Value::Int(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "'()"),
        }
    }
}
