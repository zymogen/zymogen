mod error;
mod lexer;
mod parser;

pub use error::{Error, ErrorKind};
pub use lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub use parser::{Keyword, List, Parser, Sexp};

pub fn lex<S: AsRef<str>>(s: S) -> Result<Vec<Token>, String> {
    match Lexer::new(s.as_ref()).lex() {
        Ok(v) => Ok(v),
        Err(e) => Err(e.message(s)),
    }
}

pub fn parse<S: AsRef<str>>(s: S) -> Result<Vec<Sexp>, String> {
    match Parser::new(s.as_ref()).parse() {
        Ok(v) => Ok(v),
        Err(e) => Err(e.message(s)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_and_iter() {
        let input = "(lambda (x) y)";
        let parsed = parse(input).unwrap();
        let parsed = parsed[0].as_list().unwrap();
        let expected = vec![
            Sexp::Keyword(Keyword::Lambda),
            Sexp::List(List::Cons(
                Box::new(Sexp::Identifier("x".to_string())),
                Box::new(List::Nil),
            )),
            Sexp::Identifier("y".to_string()),
        ];
        assert_eq!(
            parsed.into_iter().collect::<Vec<&Sexp>>(),
            expected.iter().collect::<Vec<&Sexp>>(),
            "List::into_iter()"
        );
        assert_eq!(
            &expected.into_iter().collect::<parser::List>(),
            parsed,
            "List::from_iter()"
        );
    }
}
