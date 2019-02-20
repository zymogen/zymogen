// //! Transform [`hir::PrimitiveExpr`] into [`mir::Expr`]
// use super::*;
// use super::sexp::Ty;
// use super::{hir::*, mir::*};

// fn lower_lambda(mut exp: LambdaExpr) -> Expr {
//     match exp.args.len() {
//         0 => Expr::Lambda(String::new(), Box::new(lower_exp(as_prim(exp.body.remove(0))))),
//         1 => Expr::Lambda(exp.args.remove(0), Box::new(lower_exp(as_prim(exp.body.remove(0))))),
//         _ => Expr::Lambda(exp.args.remove(0), Box::new(lower_lambda(exp)))
//     }
// }

// fn lower_app(mut exp: CallExpr) -> Expr {
//     match exp.rands.len() {
//         1 =>  Expr::App(Box::new(lower_exp(as_prim(*exp.rator))), Box::new(lower_exp(as_prim(exp.rands.pop().unwrap())))),
//         _ => {
//             let arg = exp.rands.pop().unwrap();
//             Expr::App(Box::new(lower_app(exp)), Box::new(lower_exp(as_prim(arg))))
//         }
//     }

   
// }

// fn as_prim(exp: Expression) -> PrimitiveExpr {
//     match exp {
//         Expression::Primitive(e) => e,
//         _ => panic!("There should only be primitive expressions here!"),
//     }
// }

// pub fn lower_exp(exp: PrimitiveExpr) -> Expr {
//     match exp {
//         PrimitiveExpr::Lambda(expr) => lower_lambda(expr),
//         PrimitiveExpr::Call(expr) => lower_app(expr),
//         PrimitiveExpr::Literal(sexp) => match sexp {
//             Sexp::Boolean(b) => Expr::Val(Value::Bool(b)),
//             Sexp::Literal(s) => Expr::Val(Value::Str(s)),
//             Sexp::Integer(i) => Expr::Val(Value::Int(i)),
//             _ => unimplemented!(),
//         },
//         PrimitiveExpr::Variable(s) => Expr::Var(s),
//         _ => unimplemented!(),
//     }
// }