use super::*;
use super::ir::{mir::Expr, bytecode::Operation};

#[derive(Debug)]
pub struct Context<'t> {
    symbols: &'t mut SymbolTable,
    locals: Vec<String>,
    code: Vec<Operation>,
    constants: Vec<Value>,
}

impl<'t> Context<'t> {
    pub fn from(symbols: &'t mut SymbolTable) -> Context<'t> {
        Context {
            symbols,
            locals: Vec::new(),
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    fn find_var(&self, s: String) -> Operation {
        match self.locals.iter().position(|item| item == &s) {
            Some(idx) => Operation::Bound(idx),
            None => Operation::Var(s),
        }
    }

    
    pub fn compile(&mut self, expr: Expr) {
        match expr {
            Expr::Var(s) => self.code.push(self.find_var(s)),
            Expr::Val(v) | Expr::Quote(v) => {
                let idx = self.constants.len();
                self.constants.push(v);
                self.code.push(Operation::Constant(idx));
            },
            Expr::App(rator, rands) => {
                let arity = rands.len();
                rands.into_iter().for_each(|r| self.compile(r));
                self.compile(*rator);
                self.code.push(Operation::Call(arity));
            },
            Expr::Let(var, val, body) => {
                let idx = self.locals.len();
                self.locals.push(var);                
                self.compile(*val);
                self.code.push(Operation::Bind(idx));
                self.compile(*body);
            },
            _ => unimplemented!()
        }
        //println!("{:#?} {:?} {:?}", self.code, self.locals, self.constants);
    }
}

