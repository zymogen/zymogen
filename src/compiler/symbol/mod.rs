use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Symbol(usize);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$symbol-{}", self.0)
    }
}

/// A symbol interning table that allows for constant-time lookups
/// 
#[derive(Debug)]
pub struct SymbolTable {
    inner: HashMap<String, usize>,
    v: Vec<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            inner: HashMap::new(),
            v: Vec::new(),
        }
    }

    pub fn insert(&mut self, sym: String) -> Symbol {
        match self.inner.get(&sym) {
            Some(id) => Symbol(*id),
            None => {
                let id = self.v.len();
                self.inner.insert(sym.clone(), id);
                self.v.push(sym);
                Symbol(id)
            }
        }
    }

    pub fn insert_borrowed(&mut self, s: &str) -> Symbol {
        match self.inner.get(s) {
            Some(id) => Symbol(*id),
            None => {
                let id = self.v.len();
                self.inner.insert(s.to_string(), id);
                self.v.push(s.to_string());
                Symbol(id)
            }
        }
    }

    pub fn get(&self, sym: Symbol) -> Option<&str> {
        self.v.get(sym.0).map(|s| s.as_ref())
    }

    pub fn own(&self, sym: Symbol) -> String {
        self.v[sym.0].clone()
    }

    pub fn gensym(&mut self) -> Symbol {
        let id = self.v.len();
        let mut s = format!("$g{}", id);

        let mut i: usize = 0;
        while self.inner.contains_key(&s) {
            s = format!("$g{}~{}", id, i);
            i += 1;
        }
        self.inner.insert(s.clone(), id);
        self.v.push(s);
        Symbol(id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gensyms() {
        let mut table = SymbolTable::new();

        let gs1 = table.gensym();
        let gs2 = table.gensym();
        assert!(gs1 != gs2);
        assert_eq!(table.get(gs2), Some("$g1"));
    }

}
