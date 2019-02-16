use std::fmt;
use std::iter::FromIterator;
use std::iter::Iterator;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Sexp {
    Boolean(bool),
    Integer(i64),
    Identifier(String),
    Literal(String),
    Keyword(Keyword),
    List(List),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum List {
    Cons(Box<Sexp>, Box<List>),
    Nil,
}
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Keyword {
    Quote,
    Lambda,
    If,
    Set,
    Begin,
    Cond,
    And,
    Or,
    Case,
    Let,
    Letstar,
    Letrec,
    Do,
    Delay,
    Quasiquote,
    Else,
    Define,
    Unquote,
    UnquoteAt,
}

impl Sexp {
    pub fn as_ident(&self) -> Option<&String> {
        match self {
            Sexp::Identifier(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&List> {
        match self {
            Sexp::List(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_keyword(&self) -> Option<Keyword> {
        match self {
            Sexp::Keyword(kw) => Some(*kw),
            _ => None,
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut ptr = self;
        loop {
            match ptr {
                List::Cons(car, cdr) => {
                    match cdr.as_ref() {
                        List::Nil => write!(f, "{}", car)?,
                        List::Cons(_, _) => write!(f, "{} ", car)?,
                    }
                    ptr = cdr.as_ref();
                }
                List::Nil => break,
            }
        }
        write!(f, ")")
    }
}

impl fmt::Display for Sexp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Sexp::*;
        match self {
            Literal(s) => write!(f, "\"{}\"", s),
            Integer(i) => write!(f, "{}", i),
            Identifier(i) => write!(f, "{}", i),
            List(super::List::Nil) => write!(f, "'()"),
            List(inner) => write!(f, "{}", inner),
            Boolean(b) => write!(f, "{}", b),
            Keyword(k) => write!(f, "{:?}", k),
        }
    }
}


/// A borrowing iterator over [`List`]
pub struct ListIterator<'l> {
    ptr: &'l List,
}


/// An owning iterator that destructively iterates over [`List`]
/// returning the value at car
pub struct ListIntoIterator {
    // Option<List> allows us to take ownership of the interior
    // value, even when we are borrowing &mut ListIntoIterator
    ptr: Option<List>,
}

impl<'l> IntoIterator for &'l List {
    type Item = &'l Sexp;
    type IntoIter = ListIterator<'l>;
    fn into_iter(self) -> Self::IntoIter {
        ListIterator { ptr: &self }
    }
}

impl IntoIterator for List {
    type Item = Sexp;
    type IntoIter = ListIntoIterator;
    fn into_iter(self) -> Self::IntoIter {
        ListIntoIterator { ptr: Some(self) }
    }
}

impl<'l> Iterator for ListIterator<'l> {
    type Item = &'l Sexp;
    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            List::Nil => None,
            List::Cons(car, cdr) => {
                self.ptr = cdr;
                Some(&*car)
            }
        }
    }
}

impl Iterator for ListIntoIterator {
    type Item = Sexp;
    fn next(&mut self) -> Option<Self::Item> {
        let (sexp, list) = self.ptr.take()?.unpack()?;
        self.ptr = Some(list);
        Some(sexp)
    }
}

impl FromIterator<Sexp> for List {
    /// This involves a 'needless' allocation to collect the items
    /// since we need to pop them off in reverse order
    fn from_iter<I: IntoIterator<Item = Sexp>>(iter: I) -> Self {
        let mut list = List::Nil;
        let mut v = iter.into_iter().collect::<Vec<Sexp>>();
        while let Some(i) = v.pop() {
            list = List::Cons(Box::new(i), Box::new(list))
        }
        list
    }
}

impl List {
    pub fn iter(&self) -> ListIterator<'_> {
        ListIterator { ptr: self }
    }

    pub fn car(&self) -> Option<&Sexp> {
        match self {
            List::Cons(car, _) => Some(&*car),
            List::Nil => None,
        }
    }

    pub fn cdr(&self) -> Option<&List> {
        match self {
            List::Cons(_, cdr) => Some(&*cdr),
            List::Nil => None
        }
    }

    /// Destructure an owned List into it's car and cdr
    pub fn unpack(self) -> Option<(Sexp, List)> {
        match self {
            List::Cons(car, cdr) => Some((*car, *cdr)),
            List::Nil => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn cons(car: Sexp, cdr: List) -> List {
        List::Cons(Box::new(car), Box::new(cdr))
    }

    fn id(s: &str) -> Sexp {
        Sexp::Identifier(s.to_string())
    }

    #[test]
    fn list_from_iter() {
        let list = vec![
            Sexp::Keyword(Keyword::Lambda),
            Sexp::List(cons(id("x"), List::Nil)),
            id("y"),
        ];
        let expected = cons(
            Sexp::Keyword(Keyword::Lambda),
            cons(
                Sexp::List(cons(id("x"), List::Nil)),
                cons(id("y"), List::Nil),
            ),
        );
        assert_eq!(list.into_iter().collect::<List>(), expected);
    }

    #[test]
    fn list_into_iter() {
        let list = cons(id("cons"), cons(id("x"), cons(id("y"), List::Nil)));
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(id("cons")));
        assert_eq!(iter.next(), Some(id("x")));
        assert_eq!(iter.next(), Some(id("y")));
        assert_eq!(iter.next(), None);
    }
}
