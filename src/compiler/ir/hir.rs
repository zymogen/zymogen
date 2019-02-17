use super::super::*;

pub type Sequence = Vec<Expression>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Derived(DerivedExpr),
    Primitive(PrimitiveExpr),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LetBindings {
    pub var: String,
    pub expr: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LetExpr {
    Let(Vec<LetBindings>, Sequence),
    NamedLet(String, Vec<LetBindings>, Sequence),
}

/// Expressions that are not semantically primitive, e.g. they can be expressed
/// in terms of [`PrimitiveExpr`]'s.
#[derive(PartialEq, PartialOrd, Debug)]
pub enum DerivedExpr {
    Let(LetExpr),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LambdaExpr {
    pub args: Vec<String>,
    pub rest: Option<String>,
    pub body: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ProcedureCall {
    pub rator: Box<Expression>,
    pub rands: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
/// Semantic primitives
pub enum PrimitiveExpr {
    Literal(Sexp),
    Variable(String),
    Quotation(Sexp),
    Call(ProcedureCall),
    Lambda(LambdaExpr),
    Conditional,
    Assignment,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Literal {
    Quote(Box<Literal>),
    Boolean(bool),
    Integer(i64),
    String(String),
    Identifier(String),
    List(Vec<Literal>),
}
