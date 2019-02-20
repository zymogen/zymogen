//! Desugaring of HIR to MIR. MIR will then be the input language for A-normalization

use super::hir::*;
use super::mir::{Expr, Value};
use super::*;

fn desugar_bindings(mut args: Vec<String>, mut vals: Vec<Expr>, body: Expr, depth: u32) -> Expr {
    if args.is_empty() {
        body
    } else {
        Expr::Let(args.remove(0), Box::new(vals.remove(0)), Box::new(desugar_bindings(args, vals, body, depth + 1)))
    }
}

/// Desugar a let expression with multiple bindings into 
/// nested let expressions
/// ```skip
/// (let ((x 0)
///       (y 1))
///     (cons x y))
/// ===>
/// (let ((x 0)) (let ((y 1)) (cons x y)))
fn desugar_let(letexpr: LetExpr) -> Expr {
    match letexpr {
        LetExpr::Let(bind, body) => {
            let mut args = Vec::new();
            let mut rands = Vec::new();
            bind.into_iter().for_each(|bind| {
                args.push(bind.var);
                rands.push(bind.expr);
            });

            desugar_bindings(args, rands.into_iter().map(desugar).collect(), desugar_begin(body), 0)
        }
        LetExpr::NamedLet(_name, _bind, _body) => unimplemented!(),
        LetExpr::LetRec(bind, body) => {
            let mut args = Vec::new();
            let rands = (0..bind.len()).map(|_| Expr::Val(Value::Bool(false))).collect();
            let mut expanded = bind.into_iter().map(|bind| {
                args.push(bind.var.clone());
                Expression::Assignment(bind.var, Box::new(bind.expr))
            }).collect::<Vec<Expression>>();

            expanded.extend(body);
            println!("{:?} {:?}", args, rands);
            desugar_bindings(args, rands, desugar_begin(expanded), 0)
        }
    }
}

/// desugar a begin statement into let bindings
/// ```skip
/// (begin
///     exp1
///     ...
///     expN
/// )
/// ===>
/// (let (($t0 exp1) ... ($tN-1 expN-1)) expN)
fn desugar_begin(mut exprs: Sequence) -> Expr {
    if exprs.len() == 1 {
        desugar(exprs.remove(0))
    } else {
        let mut exprs = exprs.into_iter().map(desugar).collect::<Vec<Expr>>();
        let body = exprs.pop().unwrap();
        let vars = (0..exprs.len()).map(|i| format!("~s{}", i)).collect();
        desugar_bindings(vars, exprs, body, 0)
    }
}

fn desugar_cond(mut clauses: Vec<CondClause>, else_clause: Option<Sequence>) -> Expr {
    if !clauses.is_empty() {
        let fst = clauses.remove(0);
        Expr::If(
            Box::new(desugar(*fst.test)),
            Box::new(desugar(Expression::Begin(fst.body))),
            Some(Box::new(desugar_cond(clauses, else_clause))),
        )
    } else if let Some(mut seq) = else_clause {
        match seq.len() {
            0 => Expr::Val(Value::Bool(false)),
            1 => desugar(seq.remove(0)),
            _ => desugar(Expression::Begin(seq)),
        }
    } else {
        Expr::Val(Value::Bool(false))
    }
}

fn desugar_and(mut body: Sequence) -> Expr {
    if !body.is_empty() {
        Expr::If(
            Box::new(desugar(body.remove(0))),
            Box::new(desugar_and(body)),
            Some(Box::new(Expr::Val(Value::Bool(false)))),
        )
    } else {
        Expr::Val(Value::Bool(false))
    }
}

fn desugar_or(mut body: Sequence) -> Expr {
    if !body.is_empty() {
        Expr::If(
            Box::new(desugar(body.remove(0))),
            Box::new(Expr::Val(Value::Bool(true))),
            Some(Box::new(desugar_or(body))),
        )
    } else {
        Expr::Val(Value::Bool(false))
    }
}

fn desugar_lambda(lambda: LambdaExpr) -> Expr {
    Expr::Lambda(
        lambda.args,
        lambda.rest,
        vec![desugar_begin(lambda.body)],
    )
}

fn desugar_if(test: Expression, csq: Expression, alt: Option<Box<Expression>>) -> Expr {
    Expr::If(
        Box::new(desugar(test)),
        Box::new(desugar(csq)),
        alt.map(|exp| Box::new(desugar(*exp))),
    )
}

fn desugar_call(rator: Expression, rands: Sequence) -> Expr {
    Expr::App(
        Box::new(desugar(rator)),
        rands.into_iter().map(desugar).collect(),
    )
}
fn desugar_assignment(var: String, val: Expression) -> Expr {
    Expr::Set(var, Box::new(desugar(val)))
}

pub fn desugar(expr: Expression) -> Expr {
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
        Expression::Literal(Sexp::Literal(s)) => Expr::Val(Value::Str(s)),
        Expression::Literal(Sexp::Boolean(s)) => Expr::Val(Value::Bool(s)),
        Expression::Literal(Sexp::Integer(i)) => Expr::Val(Value::Int(i)),
        Expression::Literal(_) => unimplemented!(),
        Expression::Variable(s) => Expr::Var(s),
        Expression::Quotation(inner) => Expr::Quote(inner),
    }
}
