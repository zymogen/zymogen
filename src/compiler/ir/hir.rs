//! Definitions of the higher-intermediate-representation abstract syntax trees
//!
//! These are directly parsed from the input, and in the transformation down to
//! MIR, all derived expressions will be converted into primitive expressions
//! and the AST will be simplified
use super::super::Keyword;
use super::Value;

pub type Sequence = Vec<Expression>;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Expression {
    Keyword(Keyword),
    Let(LetExpr),
    Begin(Sequence),
    Cond(Vec<CondClause>, Option<Sequence>),
    And(Sequence),
    Or(Sequence),
    Quasiquoted(u32, Box<Expression>),
    Literal(Value),
    Variable(String),
    Quotation(Value),
    Call(Box<Expression>, Sequence),
    Lambda(LambdaExpr),
    If(Box<Expression>, Box<Expression>, Option<Box<Expression>>),
    Assignment(String, Box<Expression>),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LetBindings {
    pub var: String,
    pub expr: Expression,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum LetExpr {
    Let(Vec<LetBindings>, Sequence),
    LetRec(Vec<LetBindings>, Sequence),
    NamedLet(String, Vec<LetBindings>, Sequence),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LambdaExpr {
    pub args: Vec<String>,
    pub rest: Option<String>,
    pub body: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct CondClause {
    pub test: Box<Expression>,
    pub body: Sequence,
}
