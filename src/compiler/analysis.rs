use super::ir::hir::{self, Expression::*, *};
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
    Ok(Expression::Primitive(PrimitiveExpr::Lambda(LambdaExpr {
        args: params,
        rest,
        body,
    })))
}

fn analyze_let_bindings(bindings: List) -> Vec<LetBindings> {
    bindings
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
    let (bindings, body) = exprs.unpack()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Derived(DerivedExpr::Let(LetExpr::Let(
        bind, body,
    ))))
}

fn analyze_letrec(exprs: List) -> Result<Expression, Error> {
    let (name, bindings, body) = exprs.unpack2()?;
    let name = name.ident()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Derived(DerivedExpr::Let(LetExpr::NamedLet(
        name, bind, body,
    ))))
}

fn analyze_call(func: Expression, rest: List) -> Result<Expression, Error> {
    let rands = analyze_sequence(rest)?;
    Ok(Expression::Primitive(PrimitiveExpr::Call(CallExpr {
        rator: Box::new(func),
        rands,
    })))
}

fn analyze_if(rest: List) -> Result<Expression, Error> {
    let (test, csq, alt) = rest.unpack2()?;
    let test = Box::new(analyze(test)?);
    let csq = Box::new(analyze(csq)?);
    let alt = match alt.unpack() {
        Ok((sexp, _)) => Some(Box::new(analyze(sexp)?)),
        Err(_) => None,
    };
    Ok(Expression::Primitive(PrimitiveExpr::If(IfExpr {
        test,
        csq,
        alt,
    })))
}

fn analyze_cond(rest: List) -> Result<Expression, Error> {
    let mut clauses = Vec::new();
    let mut else_clause = None;
    let mut next = rest;
    loop {
        let (car, cdr) = match next.unpack() {
            Ok((car, cdr)) => (car, cdr),
            Err(_) => break,
        };

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

    Ok(Expression::Derived(DerivedExpr::Cond(CondExpr {
        clauses,
        else_clause,
    })))
}

fn analyze_assignment(exprs: List) -> Result<Expression, Error> {
    let (var, exp, _) = exprs.unpack2()?;
    Ok(Expression::Primitive(PrimitiveExpr::Assignment(
        Assignment {
            var: var.ident()?,
            exp: Box::new(analyze(exp)?),
        },
    )))
}

fn analyze_define(exprs: List) -> Result<Expression, Error> {
    let (var, rest) = exprs.unpack()?;

    match var {
        Sexp::List(List::Cons(f, args)) => {
            // Easiest way to handle this is to construct a mock lambda body
            // and then pass to the analyze_lambda function
            let lambda_body = List::Cons(Box::new(Sexp::List(*args)), Box::new(rest));
            Ok(Expression::Primitive(PrimitiveExpr::Assignment(
                Assignment {
                    var: f.as_ident()?.clone(),
                    exp: Box::new(analyze_lambda(lambda_body)?),
                },
            )))
        }

        Sexp::Identifier(s) => Ok(Expression::Primitive(PrimitiveExpr::Assignment(
            Assignment {
                var: s,
                exp: Box::new(analyze(rest.unpack()?.0)?),
            },
        ))),
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
    //             Sexp::Keyword(Keyword::UnquoteAt) => vec.push(analyze_quasiquote(depth - 1, *cdar)?),
    //             Sexp::Keyword(Keyword::Quasiquote) => vec.push(analyze_quasiquote(depth + 1, *cdar)?),
    //             _ => {
    //                 vec.push(analyze(*caar)?);
    //                 vec.extend(analyze_quasiquote(depth, cdar));
    //             }
    //         }
    //     },
    //     Sexp::Keyword(Keyword::Quasiquote) => analyze_quasiquote(depth + 1, cdr),
    //     _ => analyze(car),
    // };
    // vec.push(expr);
    // Ok(Expression::Derived(DerivedExpr::Quasiquoted(depth, vec)))
    unimplemented!()
}

#[inline]
fn analyze_list(exprs: List) -> Result<Expression, Error> {
    let (car, cdr) = exprs.unpack()?;
    let f = analyze(car)?;
    match f {
        Primitive(PrimitiveExpr::Literal(sexp)) => match sexp {
            Sexp::Keyword(Keyword::Lambda) => analyze_lambda(cdr),
            Sexp::Keyword(Keyword::Let) | Sexp::Keyword(Keyword::Letstar) => analyze_let(cdr),
            Sexp::Keyword(Keyword::Letrec) => analyze_letrec(cdr),
            Sexp::Keyword(Keyword::Begin) => Ok(Expression::Derived(DerivedExpr::Begin(
                analyze_sequence(cdr)?,
            ))),
            Sexp::Keyword(Keyword::If) => analyze_if(cdr),
            Sexp::Keyword(Keyword::Cond) => analyze_cond(cdr),
            Sexp::Keyword(Keyword::Define) => analyze_define(cdr),
            Sexp::Keyword(Keyword::Set) => analyze_assignment(cdr),
            Sexp::Keyword(Keyword::And) => Ok(Expression::Derived(DerivedExpr::And(
                analyze_sequence(cdr)?,
            ))),
            Sexp::Keyword(Keyword::Or) => {
                Ok(Expression::Derived(DerivedExpr::Or(analyze_sequence(cdr)?)))
            }
            Sexp::Keyword(Keyword::Quote) => Ok(Expression::Primitive(PrimitiveExpr::Quotation(
                cdr.unpack()?.0,
            ))),
            Sexp::Keyword(Keyword::Quasiquote) => analyze_quasiquote(1, cdr),
            Sexp::Identifier(func) => analyze_call(Primitive(PrimitiveExpr::Variable(func)), cdr),
            _ => panic!("Invalid start of list {}", sexp),
        },
        Primitive(PrimitiveExpr::Lambda(_)) | Primitive(PrimitiveExpr::Call(_)) => {
            analyze_call(f, cdr)
        }
        Primitive(PrimitiveExpr::Variable(_)) => analyze_call(f, cdr),
        _ => panic!("Invalid expr! {:?}", f),
    }
}

#[inline]
fn analyze_sequence(exprs: List) -> Result<Sequence, Error> {
    exprs
        .into_iter()
        .map(analyze)
        .collect::<Result<Vec<Expression>, Error>>()
}

#[inline]
pub fn analyze(expr: Sexp) -> Result<Expression, Error> {
    match expr {
        Sexp::Boolean(_) | Sexp::Integer(_) | Sexp::Literal(_) | Sexp::Keyword(_) => {
            Ok(Primitive(PrimitiveExpr::Literal(expr)))
        }
        Sexp::Identifier(s) => Ok(Primitive(PrimitiveExpr::Variable(s))),
        Sexp::List(list) => analyze_list(list),
    }
}
