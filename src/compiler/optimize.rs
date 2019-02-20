use super::mir::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub struct Gensym {
    store: BTreeMap<String, u32>,
    v: Vec<String>,
}

impl Gensym {
    pub fn new() -> Gensym {
        Gensym {
            store: BTreeMap::new(),
            v: Vec::new(),
        }
    }

    pub fn contains(&self, s: &str) -> bool {
        self.store.contains_key(s)
    }

    pub fn store(&mut self, s: String) -> u32 {
        match self.store.get(&s) {
            Some(id) => *id,
            None => {
                let id = self.v.len() as u32;
                self.store.insert(s.clone(), id);
                self.v.push(s);
                id
            }
        }
    }
}

pub struct Context {
    names: Gensym,
}

impl Context {
    pub fn new() -> Self {
        Context {
            names: Gensym::new(),
        }
    }
}

fn side_effects(exp: &Expr) -> bool {
    match exp {
        Expr::Set(_, _) => true,
        Expr::If(a, b, Some(c)) => side_effects(a) | side_effects(b) | side_effects(c),
        Expr::If(a, b, None) => side_effects(a) | side_effects(b),
        Expr::Let(_, a, b) => side_effects(a) | side_effects(b),
        Expr::Lambda(_, _, body) => side_effects(body),
        Expr::App(rator, rands) => {
            side_effects(rator) | rands.iter().fold(false, |acc, x| acc | side_effects(x))
        }
        Expr::Var(_) | Expr::Val(_) | Expr::Quote(_) => false,
    }
}

/// Extract a list of bound variables
fn extract_bound(exp: &Expr) -> Vec<&String> {
    let mut v = Vec::new();
    match exp {
        Expr::Var(s) => v.push(s),
        Expr::Let(s, _, body) => {
            v.push(s);
            v.extend(extract_bound(body));
        }
        Expr::Lambda(a, Some(b), body) => {
            v.extend(a);
            v.push(b);
            v.extend(extract_bound(body));
        }
        Expr::Lambda(a, None, body) => {
            v.extend(a);
            v.extend(extract_bound(body));
        }
        Expr::App(rator, rands) => {
            v.extend(extract_bound(rator));
            v.extend(rands.iter().fold(Vec::new(), |mut acc, x| {
                acc.extend(extract_bound(x));
                acc
            }));
        }
        Expr::If(a, b, Some(c)) => {
            v.extend(extract_bound(a));
            v.extend(extract_bound(b));
            v.extend(extract_bound(c));
        }
        Expr::If(a, b, None) => {
            v.extend(extract_bound(a));
            v.extend(extract_bound(b));
        }
        Expr::Set(s, exp) => {
            v.push(s);
            v.extend(extract_bound(exp))
        }
        _ => {}
    }
    v
}

/// Extract a list of used variables
fn extract_ref(exp: &Expr) -> Vec<&String> {
    let mut v = Vec::new();
    match exp {
        Expr::Var(s) => v.push(s),

        Expr::Let(_, bind, body) => {
            v.extend(extract_ref(bind));
            v.extend(extract_ref(body));
        }
        Expr::Lambda(_, _, body) => {
            v.extend(extract_ref(body));
        }
        Expr::App(rator, rands) => {
            v.extend(extract_ref(rator));
            v.extend(rands.iter().fold(Vec::new(), |mut acc, x| {
                acc.extend(extract_ref(x));
                acc
            }));
        }
        Expr::If(a, b, Some(c)) => {
            v.extend(extract_ref(a));
            v.extend(extract_ref(b));
            v.extend(extract_ref(c));
        }
        Expr::If(a, b, None) => {
            v.extend(extract_ref(a));
            v.extend(extract_ref(b));
        }
        Expr::Set(s, exp) => {
            v.push(s);
            v.extend(extract_ref(exp));
        }
        Expr::Quote(_) | Expr::Val(_) => {}
    }
    v
}

pub fn eliminate_bindings(exp: Expr, ctx: &mut Context) -> Expr {
    let mut next = &exp;
    let mut effect = BTreeMap::new();
    //let mut census = BTreeMap::new();
    // Traverse through the let bindings
    while let Expr::Let(var, val, body) = next {
        ctx.names.store(var.clone());
        effect.insert(var.clone(), side_effects(val));
        next = &*body;
    }

    let referenced = extract_ref(&exp).into_iter().collect::<BTreeSet<_>>();
    let bound = extract_bound(&exp).into_iter().collect::<BTreeSet<_>>();

    let free: BTreeSet<_> = referenced.difference(&bound).collect();
    let unused: BTreeSet<_> = bound.difference(&referenced).collect();

    println!("{:?}", ctx.names);
    println!("referenced {:?}", referenced);
    println!("bound {:?}", bound);
    println!("free {:?}", free);
    println!("unused bindings {:?}", unused);
    println!("side effect let bound? {:?}", effect);

    let mut eliminate = BTreeSet::new();
    for &k in unused {
        if let Some(false) = effect.get(k) {
            println!("binding {} can be eliminated", k);
            eliminate.insert(k);
        }
    }

    exp
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_side_effects() {
        assert_eq!(
            side_effects(&Expr::Set(
                String::new(),
                Box::new(Expr::Val(Value::Bool(true)))
            )),
            true
        );
        assert_eq!(
            side_effects(&Expr::Lambda(
                vec![String::new()],
                None,
                Box::new(Expr::Val(Value::Bool(true)))
            )),
            false
        );
    }
}
