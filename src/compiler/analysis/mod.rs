use super::super::syntax::Expression as Sexp;
use super::super::syntax::Keyword::*;
use super::ir::hir::{self, Expression::*, *};


pub fn analyze_lambda(exprs: Vec<Sexp>) -> Expression {
    Expression::Primitive(PrimitiveExpr::Lambda)
}

pub fn analyze_definition(exprs: Vec<Sexp>) -> Expression {
    Expression::Primitive(PrimitiveExpr::Lambda)
}

// fn analyze_let_bindings(exprs: Vec<Sexp>) -> Vec<LetBindings> {
//     let mut bindings = Vec::new();
//     let mut iter =  exprs.into_iter();
//     while let Some(Sexp::List(v)) = iter.next() {
//         bindings.push(LetBindings {
//             var: v[0].as_ident().unwrap(),
//             expr: v[1],
//         });
//     }
//     bindings
// }

// pub fn analyze_let(exprs: Vec<Sexp>) -> Expression {
//     let e = match exprs[0] {
//         Sexp::Keyword(Let) => LetExpr::Let()
//     };
//     Expression::Derived(DerivedExpr::Let(letexpr))
// }

pub fn analyze_list(exprs: Vec<Sexp>) -> Expression {
    match exprs.get(0).unwrap() {
      //  Sexp::Keyword(Begin) => analyze_begin(exprs),
        Sexp::Keyword(Lambda) => analyze_lambda(exprs),
        Sexp::Keyword(Define) => analyze_definition(exprs),
       // Sexp::Keyword(And) | Sexp::Keyword(Or) => analyze_binary(exprs),
     //   Sexp::Keyword(Let) | Sexp::Keyword(Letstar) | Sexp::Keyword(Letrec) => analyze_let(exprs),
        Sexp::Identifier(_) => Expression::Primitive(PrimitiveExpr::Call),
        unknown => panic!("{:?}", unknown)
    }
}

pub fn analyze(expr: Sexp) -> Expression {
    match expr {
        Sexp::Boolean(b) => Primitive(PrimitiveExpr::Literal(Literal::Boolean(b))),
        Sexp::Integer(b) => Primitive(PrimitiveExpr::Literal(Literal::Integer(b))),
        Sexp::Literal(s) => Primitive(PrimitiveExpr::Literal(Literal::String(s))),
        Sexp::Identifier(s) => Primitive(PrimitiveExpr::Variable(s)),
        Sexp::Keyword(keyword) => panic!("Keyword {:?} at top level!", keyword),
        Sexp::List(vec) => analyze_list(vec),
    }
}