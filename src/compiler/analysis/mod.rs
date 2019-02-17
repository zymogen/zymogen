use super::*;
use super::sexp::Ty;
use super::ir::hir::{self, Expression::*, *};
pub mod desugar;

pub fn analyze_lambda(exprs: List) -> Result<Expression, Error> {
    let (args, body) = exprs.unpack()?;
    let body = analyze_sequence(body)?;
    let mut params = Vec::new();
    let mut rest = None;

    match args {
        Sexp::List(List::Nil) => {},
        Sexp::List(inner) => {
            let mut iter = inner.into_iter();
            loop {
                let n = iter.next();
                match n {
                    Some(Sexp::Keyword(Keyword::Dot)) => {
                        rest = Some(iter.next().ok_or(Error::Arity)?.ident()?);
                    },
                    Some(Sexp::Identifier(s)) => {
                        params.push(s);
                    },
                    Some(exp) => return Err(Error::WrongType(Ty::Identifier, exp.ty())),
                    None => break,
                }
            }
        },
        Sexp::Identifier(rest_id) => {
            rest = Some(rest_id);
        },
        _ => return Err(Error::WrongType(Ty::Identifier, args.ty())),
    }
    Ok(Expression::Primitive(PrimitiveExpr::Lambda(LambdaExpr {
        args: params,
        rest,
        body,
    })))
}

pub fn analyze_let_bindings(bindings: List) -> Vec<LetBindings> {
    bindings.into_iter().filter_map(|bind| {
        match bind{
            Sexp::List(List::Nil) => None,
            Sexp::List(l) => {
                let (car, cadr, _) = l.unpack2().ok()?;
                Some(LetBindings {
                    var: car.ident().ok()?,
                    expr: analyze(cadr).ok()?,
                })                
            }
            _ => None
        }}).collect()
}

pub fn analyze_let(exprs: List) -> Result<Expression, Error> {
    let (bindings, body) = exprs.unpack()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Derived(DerivedExpr::Let(LetExpr::Let(bind, body))))
}


pub fn analyze_letrec(exprs: List) -> Result<Expression, Error> {
    let (name, bindings, body) = exprs.unpack2()?;
    let name = name.ident()?;
    let bind = analyze_let_bindings(bindings.list()?);
    let body = analyze_sequence(body)?;
    Ok(Expression::Derived(DerivedExpr::Let(LetExpr::NamedLet(name, bind, body))))
}

pub fn analyze_call(func: Expression, rest: List) -> Result<Expression, Error> {
    let rands = analyze_sequence(rest)?;
    Ok(Expression::Primitive(PrimitiveExpr::Call(ProcedureCall {
        rator: Box::new(func),
        rands,
    })))
}

pub fn analyze_list(exprs: List) -> Result<Expression, Error> {
    let (car, cdr) = exprs.unpack()?;
    let f = analyze(car)?;
    match f {
        Primitive(PrimitiveExpr::Literal(sexp)) => {
            match sexp {
                Sexp::Keyword(Keyword::Lambda) => analyze_lambda(cdr),
                Sexp::Keyword(Keyword::Let) | Sexp::Keyword(Keyword::Letstar) => analyze_let(cdr),
                Sexp::Keyword(Keyword::Letrec) => analyze_letrec(cdr),
                Sexp::Keyword(Keyword::Quote) => Ok(Expression::Primitive(PrimitiveExpr::Quotation(cdr.unpack()?.0))),
                Sexp::Identifier(func) => analyze_call(Primitive(PrimitiveExpr::Variable(func)), cdr),
                _ => panic!("Invalid expr!"),
            }
        },
        Primitive(PrimitiveExpr::Lambda(_)) 
            | Primitive(PrimitiveExpr::Call(_))  => analyze_call(f, cdr),
            | Primitive(PrimitiveExpr::Variable(_)) => analyze_call(f, cdr),
        _ => panic!("Invalid expr! {:?}", f),
    }
}

pub fn analyze_sequence(exprs: List) -> Result<Sequence, Error> {
    exprs.into_iter().map(analyze).collect::<Result<Vec<Expression>, Error>>()
}

pub fn analyze(expr: Sexp) -> Result<Expression, Error> {
    match expr {
        Sexp::Boolean(_) | Sexp::Integer(_) | Sexp::Literal(_) | Sexp::Keyword(_) => Ok(Primitive(PrimitiveExpr::Literal(expr))),
        Sexp::Identifier(s) => Ok(Primitive(PrimitiveExpr::Variable(s))),
        Sexp::List(list) => analyze_list(list),
    }
}
