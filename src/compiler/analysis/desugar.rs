use super::*;

pub fn desugar_let(letexpr: LetExpr) -> PrimitiveExpr {
    match letexpr {
        LetExpr::Let(bind, body) => {
            let mut args = Vec::new();
            let mut rands = Vec::new();
            bind.into_iter().for_each(|bind| {
                args.push(bind.var);
                rands.push(bind.expr);
            });

            PrimitiveExpr::Call(ProcedureCall {
                rator: Box::new(Expression::Primitive(PrimitiveExpr::Lambda(LambdaExpr {
                    args,
                    rest: None,
                    body,
                }))),
                rands,
            })
        },
        LetExpr::NamedLet(name, bind, body) => {
            unimplemented!()
        },
    }
}

pub fn desugar(expr: Expression) -> PrimitiveExpr {
    match expr {
        Primitive(inner) => inner,
        Derived(derived) => match derived {
            DerivedExpr::Let(expr) => desugar_let(expr)
        }
    }
}