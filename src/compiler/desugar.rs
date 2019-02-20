use super::ir::hir::*;
use super::*;

/// Desugar a let expression into a lambda with application
/// ```skip
/// (let ((x 0)
///       (y 1))
///     (cons x y))
/// ===>
/// ((lambda (x y) (cons x y)) 0 1)
fn desugar_let(letexpr: LetExpr) -> Expression {
    match letexpr {
        LetExpr::Let(bind, body) => {
            let mut args = Vec::new();
            let mut rands = Vec::new();
            bind.into_iter().for_each(|bind| {
                args.push(bind.var);
                rands.push(bind.expr);
            });

            Expression::Call(
                Box::new(Expression::Lambda(LambdaExpr {
                    args,
                    rest: None,
                    body,
                })),
                rands.into_iter().map(desugar).collect(),
            )
        }
        LetExpr::NamedLet(_name, _bind, _body) => unimplemented!(),
        LetExpr::LetRec(_bind, _body) => unimplemented!(),
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
fn desugar_begin(mut exprs: Sequence) -> Expression {
    if exprs.len() == 1 {
        desugar(exprs.remove(0))
    } else {
        Expression::Call(
            Box::new(Expression::Lambda(LambdaExpr {
                args: (0..exprs.len()).map(|i| format!("$s{}", i)).collect(),
                rest: None,
                body: vec![Expression::Variable(format!("$s{}", exprs.len() - 1))],
            })),
            exprs.into_iter().map(desugar).collect(),
        )
    }
}

fn desugar_cond(mut clauses: Vec<CondClause>, else_clause: Option<Sequence>) -> Expression {
    if !clauses.is_empty() {
        let fst = clauses.remove(0);
        Expression::If(
            Box::new(desugar(*fst.test)),
            Box::new(desugar(Expression::Begin(fst.body))),
            Some(Box::new(desugar_cond(clauses, else_clause))),
        )
    } else if let Some(mut seq) = else_clause {
        match seq.len() {
            0 => Expression::Literal(Sexp::Boolean(false)),
            1 => desugar(seq.remove(0)),
            _ => desugar(Expression::Begin(seq)),
        }
    } else {
        Expression::Literal(Sexp::Boolean(false))
    }
}

fn desugar_and(mut body: Sequence) -> Expression {
    if !body.is_empty() {
        Expression::If(
            Box::new(desugar(body.remove(0))),
            Box::new(desugar_and(body)),
            Some(Box::new(Expression::Literal(Sexp::Boolean(false)))),
        )
    } else {
        Expression::Literal(Sexp::Boolean(true))
    }
}

fn desugar_or(mut body: Sequence) -> Expression {
    if !body.is_empty() {
        Expression::If(
            Box::new(desugar(body.remove(0))),
            Box::new(Expression::Literal(Sexp::Boolean(true))),
            Some(Box::new(desugar_or(body))),
        )
    } else {
        Expression::Literal(Sexp::Boolean(false))
    }
}

fn desugar_lambda(lambda: LambdaExpr) -> Expression {
    Expression::Lambda(LambdaExpr {
        args: lambda.args,
        rest: lambda.rest,
        body: vec![desugar_begin(lambda.body)],
    })
}

fn desugar_if(test: Expression, csq: Expression, alt: Option<Box<Expression>>) -> Expression {
    Expression::If(
        Box::new(desugar(test)),
        Box::new(desugar(csq)),
        alt.map(|exp| Box::new(desugar(*exp))),
    )
}

fn desugar_call(rator: Expression, rands: Sequence) -> Expression {
    Expression::Call(
        Box::new(desugar(rator)),
        rands.into_iter().map(desugar).collect(),
    )
}
fn desugar_assignment(var: String, val: Expression) -> Expression {
    Expression::Assignment(var, Box::new(desugar(val)))
}

pub fn desugar(expr: Expression) -> Expression {
    match expr {
        Expression::If(test, csq, alt) => desugar_if(*test, *csq, alt),
        Expression::Lambda(expr) => desugar_lambda(expr),
        Expression::Call(rator, rands) => desugar_call(*rator, rands),
        Expression::Assignment(var, val) => desugar_assignment(var, *val),
        Expression::Let(expr) => desugar_let(expr),
        Expression::Begin(expr) => desugar_begin(expr),
        Expression::Cond(clauses, else_clause) => desugar_cond(clauses, else_clause),
        Expression::And(body) => desugar_and(body),
        Expression::Or(body) => desugar_or(body),
        Expression::Quasiquoted(_depth, _seq) => unimplemented!(),

        // Self-evalulating expressions
        Expression::Literal(_) => expr,
        Expression::Variable(_) => expr,
        Expression::Quotation(_) => expr,
    }
}
