use super::sexp::Ty;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Error {
    WrongType(Ty, Ty),
    Arity,
    EmptyList,
    Message(String),
}
