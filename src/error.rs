use super::sexp::Ty;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Error {
    WrongType(Ty, Ty),
    Arity,
    EmptyList,
}
