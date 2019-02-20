use super::*;
use super::ir::hir::{Expression::*, *};

/// Desugar a let expression into a lambda with application
/// ```skip
/// (let ((x 0)
///       (y 1))
///     (cons x y))
/// ===>
/// ((lambda (x y) (cons x y)) 0 1)
fn desugar_let(letexpr: LetExpr) -> PrimitiveExpr {
    match letexpr {
        LetExpr::Let(bind, body) => {
            let mut args = Vec::new();
            let mut rands = Vec::new();
            bind.into_iter().for_each(|bind| {
                args.push(bind.var);
                rands.push(bind.expr);
            });

            PrimitiveExpr::Call(CallExpr {
                rator: Box::new(Expression::Primitive(PrimitiveExpr::Lambda(LambdaExpr {
                    args,
                    rest: None,
                    body,
                }))),
                rands: rands.into_iter().map(|r| Expression::Primitive(desugar(r))).collect(),
            })
        }
        LetExpr::NamedLet(name, bind, body) => unimplemented!(),
        LetExpr::LetRec(bind, body) => unimplemented!(),
    }
}

/// desugar a begin statement into an N-arity lambda
/// ```skip
/// (begin
///     exp1
///     exp2
///     exp3
/// )
/// ===>
/// ((lambda (x y z) z) exp1 exp2 exp3)
fn desugar_begin(mut exprs: Sequence) -> PrimitiveExpr {
    if exprs.len() == 1 {
        desugar(exprs.remove(0))
    } else {
        PrimitiveExpr::Call(CallExpr {
            rator: Box::new(Expression::Primitive(PrimitiveExpr::Lambda(LambdaExpr {
                args: (0..exprs.len()).map(|i| format!("$s{}", i)).collect(),
                rest: None,
                body: vec![Expression::Primitive(PrimitiveExpr::Variable(format!(
                    "$s{}",
                    exprs.len() - 1
                )))],
            }))),
            rands: exprs
                .into_iter()
                .map(|exp| Expression::Primitive(desugar(exp)))
                .collect(),
        })
    }
}

fn desugar_cond(mut expr: CondExpr) -> PrimitiveExpr {
    if !expr.clauses.is_empty() {
        let fst = expr.clauses.remove(0);
        PrimitiveExpr::If(IfExpr {
            test: Box::new(Expression::Primitive(desugar(*fst.test))),
            csq: Box::new(Expression::Primitive(desugar(Expression::Derived(
                DerivedExpr::Begin(fst.body),
            )))),
            alt: Some(Box::new(Expression::Primitive(desugar_cond(expr)))),
        })
    } else {
        if let Some(mut seq) = expr.else_clause {
            match seq.len() {
                0 => PrimitiveExpr::Literal(Sexp::Boolean(false)),
                1 => desugar(seq.remove(0)),
                _ => desugar(Expression::Derived(DerivedExpr::Begin(seq))),
            }
        } else {
            PrimitiveExpr::Literal(Sexp::Boolean(false))
        }
    }
}

fn desugar_and(mut body: Sequence) -> PrimitiveExpr {
    if !body.is_empty() {
        PrimitiveExpr::If(IfExpr {
            test: Box::new(Expression::Primitive(desugar(body.remove(0)))),
            csq: Box::new(Expression::Primitive(desugar_and(body))),
            alt: Some(Box::new(Expression::Primitive(PrimitiveExpr::Literal(
                Sexp::Boolean(false),
            )))),
        })
    } else {
        PrimitiveExpr::Literal(Sexp::Boolean(true))
    }
}

fn desugar_or(mut body: Sequence) -> PrimitiveExpr {
    if !body.is_empty() {
        PrimitiveExpr::If(IfExpr {
            test: Box::new(Expression::Primitive(desugar(body.remove(0)))),
            csq: Box::new(Expression::Primitive(PrimitiveExpr::Literal(
                Sexp::Boolean(true),
            ))),
            alt: Some(Box::new(Expression::Primitive(desugar_or(body)))),
        })
    } else {
        PrimitiveExpr::Literal(Sexp::Boolean(false))
    }
}

fn desugar_lambda(lambda: LambdaExpr) -> PrimitiveExpr {
    PrimitiveExpr::Lambda(LambdaExpr {
        args: lambda.args,
        rest: lambda.rest,
        body: vec![Expression::Primitive(desugar_begin(lambda.body))],
    })
}

fn desugar_if(cond: IfExpr) -> PrimitiveExpr {
    PrimitiveExpr::If(IfExpr {
        test: Box::new(Expression::Primitive(desugar(*cond.test))),
        csq: Box::new(Expression::Primitive(desugar(*cond.csq))),
        alt: cond
            .alt
            .map(|exp| Box::new(Expression::Primitive(desugar(*exp)))),
    })
}

fn desugar_call(call: CallExpr) -> PrimitiveExpr {
    PrimitiveExpr::Call(CallExpr {
        rator: Box::new(Expression::Primitive(desugar(*call.rator))),
        rands: call
            .rands
            .into_iter()
            .map(|exp| Expression::Primitive(desugar(exp)))
            .collect(),
    })
}
fn desugar_assignment(expr: Assignment) -> PrimitiveExpr {
    PrimitiveExpr::Assignment(Assignment {
        var: expr.var,
        exp: Box::new(Expression::Primitive(desugar(*expr.exp))),
    })
}

pub fn desugar(expr: Expression) -> PrimitiveExpr {
    match expr {
        Primitive(inner) => match inner {
            PrimitiveExpr::If(expr) => desugar_if(expr),
            PrimitiveExpr::Lambda(expr) => desugar_lambda(expr),
            PrimitiveExpr::Call(expr) => desugar_call(expr),
            PrimitiveExpr::Assignment(expr) => desugar_assignment(expr),
            _ => inner,
        },
        Derived(derived) => match derived {
            DerivedExpr::Let(expr) => desugar_let(expr),
            DerivedExpr::Begin(expr) => desugar_begin(expr),
            DerivedExpr::Cond(expr) => desugar_cond(expr),
            DerivedExpr::And(body) => desugar_and(body),
            DerivedExpr::Or(body) => desugar_or(body),
            DerivedExpr::Quasiquoted(depth, seq) => unimplemented!(),
        },
    }
}
