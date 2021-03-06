//! Transformation to administrative normal form
//! Loosely based on Matt Might's example (provided in Racket)
//! http://matt.might.net/articles/a-normalization/
//! 
//! These transformations would be significantly easier/cleaner
//! if we could use a CPS style like that presented in the article,
//! due to the relatively complicated manipulation of let bindings
use super::mir::Expr;
use super::*;

fn is_atomic(expr: &Expr) -> bool {
    match expr {
        Expr::Var(_) => true,
        Expr::Val(_) => true,
        Expr::Quote(_) => true,
        _ => false,
    }
}

fn unbind(mut args: Vec<(String, Expr)>, body: Expr, t: &mut SymbolTable) -> Expr {
    if args.is_empty() {
        body
    } else {
        let (var, val) = args.remove(0);
        Expr::Let(
            var,
            Box::new(normalize_expr(val, t)),
            Box::new(unbind(args, body, t)),
        )
    }
}

/// Lift a Let expression in a bind position to a top level
/// e.g.
/// ```skip
/// (let ((x (let ((y (+ n 1))) y)))
///     x)
/// ===>
/// (let ((y (+ n 1)))
///     (let ((x y))
///         x))
pub fn lift_let(expr: Expr) -> Expr {
    match expr {
        Expr::Let(var, val, body) => {
            if let Expr::Let(var_, val_, body_) = *val {
                let inner = lift_let(Expr::Let(var, Box::new(lift_let(*body_)), Box::new(lift_let(*body))));
                lift_let(Expr::Let(var_, Box::new(lift_let(*val_)), Box::new(inner)))
            } else {
                Expr::Let(var, val, body)
            }
        }
        _ => expr,
    }
}

pub fn normalize_expr(expr: Expr, table: &mut SymbolTable) -> Expr {
    let expr = match expr {
        Expr::Var(_) => expr,
        Expr::Val(_) => expr,
        Expr::Quote(_) => expr,
        Expr::Lambda(args, rest, body) => {
            Expr::Lambda(args, rest, Box::new(normalize_expr(*body, table)))
        }
        Expr::Let(var, val, body) => Expr::Let(
            var,
            Box::new(normalize_expr(*val, table)),
            Box::new(normalize_expr(*body, table)),
        ),
        Expr::If(test, csq, alt) => {
            if is_atomic(&test) {
                Expr::If(
                    Box::new(normalize_expr(*test, table)),
                    Box::new(normalize_expr(*csq, table)),
                    alt.map(|a| Box::new(normalize_expr(*a, table))),
                )
            } else {
                let g = table.gensym();
                let n = Expr::Var(table.own(g));
                println!("normalized if: {}", g);
                Expr::Let(
                    table.own(g),
                    Box::new(normalize_expr(*test, table)),
                    Box::new(Expr::If(
                        Box::new(n),
                        Box::new(normalize_expr(*csq, table)),
                        alt.map(|a| Box::new(normalize_expr(*a, table))),
                    )),
                )
            }
        }
        Expr::Set(var, val) => Expr::Set(var, Box::new(normalize_expr(*val, table))),
        Expr::App(rator, rands) => {
            let mut args = Vec::new();
            let mut stack = Vec::new();
            for r in rands {
                if is_atomic(&r) {
                    args.push(r);
                } else {
                    let g = table.gensym();
                    args.push(Expr::Var(table.own(g)));
                    stack.push((table.own(g), r));
                }
            }

            if is_atomic(&rator) {
                unbind(
                    stack,
                    Expr::App(Box::new(normalize_expr(*rator, table)), args),
                    table,
                )
            } else {
                let g = table.gensym();
                let n = Expr::Var(table.own(g));
                Expr::Let(
                    table.own(g),
                    Box::new(normalize_expr(*rator, table)),
                    Box::new(lift_let(unbind(stack, Expr::App(Box::new(n), args), table))),
                )
            }
        }
    };
    lift_let(expr)
}
