//! Mid-level intermediate representation
//!
//! And interesting quirk of this IR is that cons lists no longer exist,
//! and are simulated as Expr::App("cons", ...)
use super::Value;
use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Var(String),
    /// Value
    Val(Value),

    /// Let form with one variable binding and a single body expression
    /// for administrative normal form
    Let(String, Box<Expr>, Box<Expr>),
    /// Lambda form required variables, optional rest arg and body
    Lambda(Vec<String>, Option<String>, Box<Expr>),
    /// Function application with arguments
    App(Box<Expr>, Vec<Expr>),
    /// If expression
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),

    Set(String, Box<Expr>),

    Quote(Value),
}

/// Indent-level aware pretty printing
fn pp(e: &Expr, lvl: u32) -> String {
    let mut indent = (0..lvl * 4).map(|_| ' ').collect::<String>();
    let out = match e {
            Expr::Val(v) => format!("{}", v),
            Expr::Var(s) => format!("{}", s),
            Expr::Let(var, val, body) => format!(
                "(let (({} {}))\n{})",
                var,
                val,
                pp(body, lvl+1)
            ),
            Expr::Lambda(var, None, body) => format!("(λ ({})\n{})", var.join(" "), pp(body, lvl+1)),
            Expr::Lambda(var, Some(rest), body) => {
                format!("(λ ({} . {})\n{})", var.join(" "), rest, pp(body, lvl+1))
            }
            Expr::App(rator, rand) => format!(
                "({} {})",
                rator,
                rand.iter()
                    .map(|e| format!("{}", e))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expr::If(test, csq, None) => format!("(if {}\n{})", test, pp(csq, lvl+1)),
            Expr::If(test, csq, Some(alt)) => format!("(if {}\n{}\n{})", test, pp(csq, lvl + 1), pp(alt, lvl + 1)),
            Expr::Set(var, exp) => format!("(set! {}\n{})", var, pp(exp, lvl+1)),
            Expr::Quote(Value::Nil) => format!("'()"),
            Expr::Quote(exp) => format!("'{}", exp),
        };
    indent.push_str(&out);
    indent
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", pp(self, 0))
    }
}
