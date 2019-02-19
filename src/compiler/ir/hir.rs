//! Definitions of the higher-intermediate-representation abstract syntax trees
//!
//! These are directly parsed from the input, and in the transformation down to
//! MIR, all derived expressions will be converted into primitive expressions
//! and the AST will be simplified
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
    LetRec(Vec<LetBindings>, Sequence),
    NamedLet(String, Vec<LetBindings>, Sequence),
}

/// Expressions that are not semantically primitive, e.g. they can be expressed
/// in terms of [`PrimitiveExpr`]'s.
#[derive(PartialEq, PartialOrd, Debug)]
pub enum DerivedExpr {
    Let(LetExpr),
    Begin(Sequence),
    Cond(CondExpr),
    And(Sequence),
    Or(Sequence),
    Quasiquoted(u32, Sequence),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct LambdaExpr {
    pub args: Vec<String>,
    pub rest: Option<String>,
    pub body: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct CallExpr {
    pub rator: Box<Expression>,
    pub rands: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct CondExpr {
    pub clauses: Vec<CondClause>,
    pub else_clause: Option<Sequence>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct IfExpr {
    pub test: Box<Expression>,
    pub csq: Box<Expression>,
    pub alt: Option<Box<Expression>>,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct CondClause {
    pub test: Box<Expression>,
    pub body: Sequence,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Assignment {
    pub var: String,
    pub exp: Box<Expression>,
}

#[derive(PartialEq, PartialOrd, Debug)]
/// Semantic primitives
pub enum PrimitiveExpr {
    Literal(Sexp),
    Variable(String),
    Quotation(Sexp),
    Call(CallExpr),
    Lambda(LambdaExpr),
    If(IfExpr),
    Assignment(Assignment),
}

use std::fmt;

impl fmt::Display for PrimitiveExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PrimitiveExpr::*;
        match self {
            Literal(sexp) => write!(f, "{}", sexp),
            Variable(s) => write!(f, "{}", s),
            Quotation(sexp) => write!(f, "'{}", sexp),
            Call(call) => write!(
                f,
                "({} {})",
                call.rator,
                call.rands
                    .iter()
                    .map(|exp| format!("{}", exp))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Lambda(lam) => write!(
                f,
                "(λ ({}) {})",
                lam.args.join(" "),
                lam.body
                    .iter()
                    .map(|exp| format!("{}", exp))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            If(expr) => write!(
                f,
                "(if {} {} {})",
                expr.test,
                expr.csq,
                expr.alt
                    .as_ref()
                    .unwrap_or(&Box::new(Expression::Primitive(Literal(Sexp::Literal(
                        "void".to_string()
                    )))))
            ),
            Assignment(exp) => write!(f, "(set! {} {})", exp.var, exp.exp),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Derived(de) => write!(f, "{:?}", de),
            Expression::Primitive(p) => write!(f, "{}", p),
        }
    }
}
