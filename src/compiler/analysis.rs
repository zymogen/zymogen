//! Parse from raw Sexps to the HIR abstract syntax tree
use super::ir::hir::{Expression::*, *};
use super::sexp::Ty;
use super::*;

fn analyze_lambda(exprs: List) -> Result<Expression, Error> {
    let (args, body) = exprs.unpack()?;
    let body = analyze_sequence(body)?;
    let mut params = Vec::new();
    let mut rest = None;

    match args {
        Sexp::List(List::Nil) => {}
        Sexp::List(inner) => {
            let mut iter = inner.into_iter();
            loop {
                let n = iter.next();
                match n {
                    Some(Sexp::Keyword(Keyword::Dot)) => {
                        rest = Some(iter.next().ok_or(Error::Arity)?.ident()?);
                    }
                    Some(Sexp::Identifier(s)) => {
                        params.push(s);
                    }
                    Some(exp) => return Err(Error::WrongType(Ty::Identifier, exp.ty())),
                    None => break,
                }
            }
        }
        Sexp::Identifier(rest_id) => {
            rest = Some(rest_id);
        }
        _ => return Err(Error::WrongType(Ty::Identifier, args.ty())),
    }
    Ok(Expression::Lambda(LambdaExpr {
        args: params,
        rest,
        body,
    }))
}

fn analyze_let_bindings(exprs: List) -> Vec<LetBindings> {
    exprs
        .into_iter()
        .filter_map(|bind| match bind {
            Sexp::List(List::Nil) => None,
            Sexp::List(l) => {
                let (car, cadr, _) = l.unpack2().ok()?;
                Some(LetBindings {
                    var: car.ident().ok()?,
                    expr: analyze(cadr).ok()?,
                })
            }
            _ => None,
        })
        .collect()
}

fn analyze_let(exprs: List) -> Result<Expression, Error> {
    if let &Sexp::Identifier(_) = exprs.car()? {
        return analyze_namedlet(exprs);
    }
    let (bindings, body) = exprs.unpack()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Let(LetExpr::Let(bind, body)))
}

fn analyze_letrec(exprs: List) -> Result<Expression, Error> {
    if let &Sexp::Identifier(_) = exprs.car()? {
        return analyze_namedlet(exprs);
    }
    let (bindings, body) = exprs.unpack()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Let(LetExpr::LetRec(bind, body)))
}

fn analyze_namedlet(exprs: List) -> Result<Expression, Error> {
    let (name, bindings, body) = exprs.unpack2()?;
    let name = name.ident()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;

    Ok(Expression::Let(LetExpr::NamedLet(name, bind, body)))
}

fn analyze_call(func: Expression, exprs: List) -> Result<Expression, Error> {
    if exprs == List::Nil {
        Ok(Expression::Call(Box::new(func), vec![]))
    } else {
        let rands = analyze_sequence(exprs)?;
        Ok(Expression::Call(Box::new(func), rands))
    }
}

fn analyze_if(exprs: List) -> Result<Expression, Error> {
    let (test, csq, alt) = exprs.unpack2()?;
    let test = Box::new(analyze(test)?);
    let csq = Box::new(analyze(csq)?);
    let alt = match alt.unpack() {
        Ok((sexp, _)) => Some(Box::new(analyze(sexp)?)),
        Err(_) => None,
    };
    Ok(Expression::If(test, csq, alt))
}

fn analyze_cond(exprs: List) -> Result<Expression, Error> {
    let mut clauses = Vec::new();
    let mut else_clause = None;
    let mut next = exprs;
    while let Ok((car, cdr)) = next.unpack() {
        if let Ok((test, body)) = car.list()?.unpack() {
            match test {
                Sexp::Keyword(Keyword::Else) => {
                    else_clause = Some(analyze_sequence(body)?);
                    break;
                }
                _ => clauses.push(CondClause {
                    test: Box::new(analyze(test)?),
                    body: analyze_sequence(body)?,
                }),
            }
        } else {
            break;
        }
        next = cdr;
    }

    Ok(Expression::Cond(clauses, else_clause))
}

fn analyze_assignment(exprs: List) -> Result<Expression, Error> {
    let (var, exp, _) = exprs.unpack2()?;
    Ok(Expression::Assignment(
        var.ident()?,
        Box::new(analyze(exp)?),
    ))
}

fn analyze_define(exprs: List) -> Result<Expression, Error> {
    let (var, rest) = exprs.unpack()?;

    match var {
        Sexp::List(List::Cons(f, args)) => {
            // Easiest way to handle this is to construct a mock lambda body
            // and then pass to the analyze_lambda function
            let lambda_body = List::Cons(Box::new(Sexp::List(*args)), Box::new(rest));
            Ok(Expression::Assignment(
                f.as_ident()?.clone(),
                Box::new(analyze_lambda(lambda_body)?),
            ))
        }

        Sexp::Identifier(s) => Ok(Expression::Assignment(
            s,
            Box::new(analyze(rest.unpack()?.0)?),
        )),
        x => Err(Error::WrongType(Ty::Identifier, x.ty())),
    }
}

fn analyze_quasiquote(depth: u32, exprs: List) -> Result<Expression, Error> {
    // let (car, cdr) = exprs.unpack()?;
    // let mut vec = Vec::new();

    // match car {
    //     Sexp::List(List::Cons(caar, cdar)) => {
    //         match &*caar {
    //             Sexp::Keyword(Keyword::Unquote) => {
    //                 if depth == 1{
    //                     vec.extend(analyze_sequence(*cdar)?);
    //                 } else {
    //                     vec.push(analyze_quasiquote(depth - 1, *cdar)?);
    //                 }
    //             }
    //             Sexp::Keyword(Keyword::UnquoteAt) =>
    // vec.push(analyze_quasiquote(depth - 1, *cdar)?),
    // Sexp::Keyword(Keyword::Quasiquote) => vec.push(analyze_quasiquote(depth + 1,
    // *cdar)?),             _ => {
    //                 vec.push(analyze(*caar)?);
    //                 vec.extend(analyze_quasiquote(depth, cdar));
    //             }
    //         }
    //     },
    //     Sexp::Keyword(Keyword::Quasiquote) => analyze_quasiquote(depth + 1, cdr),
    //     _ => analyze(car),
    // };
    // vec.push(expr);
    // Ok(Expression::Quasiquoted(depth, vec)))
    unimplemented!()
}

#[inline]
/// Generate a thunk to delay computation
fn analyze_delay(exprs: List) -> Result<Expression, Error> {
    Ok(Expression::Lambda(LambdaExpr {
        args: vec![],
        rest: None,
        body: analyze_sequence(exprs)?,
    }))
}

#[inline]
fn analyze_list(exprs: List) -> Result<Expression, Error> {
    let (car, cdr) = exprs.unpack()?;
    let f = analyze(car)?;
    match f {
        Literal(sexp) => match sexp {
            Sexp::Keyword(Keyword::Lambda) => analyze_lambda(cdr),
            Sexp::Keyword(Keyword::Let) | Sexp::Keyword(Keyword::Letstar) => analyze_let(cdr),
            Sexp::Keyword(Keyword::Letrec) => analyze_letrec(cdr),
            Sexp::Keyword(Keyword::Begin) => Ok(Expression::Begin(analyze_sequence(cdr)?)),
            Sexp::Keyword(Keyword::If) => analyze_if(cdr),
            Sexp::Keyword(Keyword::Cond) => analyze_cond(cdr),
            Sexp::Keyword(Keyword::Define) => analyze_define(cdr),
            Sexp::Keyword(Keyword::Set) => analyze_assignment(cdr),
            Sexp::Keyword(Keyword::And) => Ok(Expression::And(analyze_sequence(cdr)?)),
            Sexp::Keyword(Keyword::Or) => Ok(Expression::Or(analyze_sequence(cdr)?)),
            Sexp::Keyword(Keyword::Quote) => Ok(Expression::Quotation(cdr.unpack()?.0)),
            Sexp::Keyword(Keyword::Quasiquote) => analyze_quasiquote(1, cdr),
            Sexp::Keyword(Keyword::Delay) => analyze_delay(cdr),
            Sexp::Identifier(func) => analyze_call(Variable(func), cdr),
            _ => panic!("Invalid start of list {}", sexp),
        },
        Lambda(_) | Call(_, _) => analyze_call(f, cdr),
        Variable(_) => analyze_call(f, cdr),
        _ => panic!("Invalid expr! {:?}", f),
    }
}

#[inline]
fn analyze_sequence(exprs: List) -> Result<Sequence, Error> {
    println!("{}", exprs);
    if exprs == List::Nil {
        return Err(Error::EmptyList);
    }
    exprs
        .into_iter()
        .map(analyze)
        .collect::<Result<Vec<Expression>, Error>>()
}

#[inline]
pub fn analyze(expr: Sexp) -> Result<Expression, Error> {
    match expr {
        Sexp::Boolean(_) | Sexp::Integer(_) | Sexp::Literal(_) | Sexp::Keyword(_) => {
            Ok(Literal(expr))
        }
        Sexp::Identifier(s) => Ok(Variable(s)),
        Sexp::List(list) => analyze_list(list),
    }
}
