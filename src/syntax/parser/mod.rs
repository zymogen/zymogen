use super::*;
mod expr;
pub use expr::{Keyword, List, Sexp};

pub struct Parser<'l> {
    lexer: Lexer<'l>,
    peek: Result<Token, Error>,
}

impl<'l> Parser<'l> {
    pub fn new(input: &'l str) -> Parser<'l> {
        let mut lexer = Lexer::new(input);
        let peek = lexer.next_token();
        Parser { lexer, peek }
    }

    fn peek(&mut self) -> Result<&Token, &Error> {
        self.peek.as_ref()
    }

    fn consume(&mut self) -> Result<Token, Error> {
        std::mem::replace(&mut self.peek, self.lexer.next_token())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        match self.consume() {
            Ok(token) => {
                if token.kind == kind {
                    Ok(token)
                } else {
                    Err(Error::from_token(&token, ErrorKind::EOF))
                }
            }
            Err(e) => Err(e),
        }
    }

    fn parse_list(&mut self) -> Result<Sexp, Error> {
        let mut vec = Vec::new();
        loop {
            match self.peek() {
                Ok(token) => {
                    match &token.kind {
                        TokenKind::RightParen => {
                            self.expect(TokenKind::RightParen)?;
                            break;
                        }
                        TokenKind::EOF => return Err(Error::from_token(token, ErrorKind::EOF)),
                        _ => {
                            // We know that the next token is NOT TokenKind::EOF
                            // so this unwrapping should never fail, since
                            // parse_expr() only returns None when TokenKind::EOF
                            // is the consumed() token. However we still need to
                            // try! the potential parsing Error
                            vec.push(self.parse_expr().expect("Unrecoverable error in parser")?);
                        }
                    }
                }
                Err(e) => return Err(e.clone()),
            }
        }

        let mut list = List::Nil;
        while let Some(exp) = vec.pop() {
            list = List::Cons(Box::new(exp), Box::new(list));
        }
        Ok(Sexp::List(list))
    }

    fn to_keyword(ident: String) -> Result<Sexp, Error> {
        use expr::Keyword::*;
        use Sexp::*;
        let res = match ident.as_ref() {
            "quote" => Keyword(Quote),
            "lambda" => Keyword(Lambda),
            "if" => Keyword(If),
            "set" => Keyword(Set),
            "begin" => Keyword(Begin),
            "cond" => Keyword(Cond),
            "and" => Keyword(And),
            "or" => Keyword(Or),
            "case" => Keyword(Case),
            "let" => Keyword(Let),
            "let*" => Keyword(Letstar),
            "letrec" => Keyword(Letrec),
            "do" => Keyword(Do),
            "delay" => Keyword(Delay),
            "quasiquote" => Keyword(Quasiquote),
            "else" => Keyword(Else),
            "define" => Keyword(Define),
            "unquote" => Keyword(Unquote),
            "unqoute-splice" | "unquoteat" => Keyword(UnquoteAt),
            _ => Identifier(ident),
        };
        Ok(res)
    }

    /// Not a very ergonomic function, but we need a way to signal that
    /// we have reached the end of input in a successful manner, i.e. Some(Ok(_))
    ///
    /// TODO: Look into refactoring the parse module to just call syntax::lex()
    /// and operate on a vec of tokens, instead of lexing on demand
    pub fn parse_expr(&mut self) -> Option<Result<Sexp, Error>> {
        use TokenKind::*;
        let token = match self.consume() {
            Err(e) => return Some(Err(e)),
            Ok(token) => token,
        };

        let expr = match token.kind {
            LeftParen => self.parse_list(),
            RightParen => Err(Error::from_token(&token, ErrorKind::Unbalanced)),
            Quote => Ok(Sexp::Keyword(Keyword::Quote)),
            Quasiquote => Ok(Sexp::Keyword(Keyword::Quasiquote)),
            Unquote => Ok(Sexp::Keyword(Keyword::Unquote)),
            UnquoteAt => Ok(Sexp::Keyword(Keyword::UnquoteAt)),
            Dot => Err(Error::from_token(&token, ErrorKind::Unbalanced)),
            Boolean(b) => Ok(Sexp::Boolean(b)),
            Integer(i) => Ok(Sexp::Integer(i)),
            Literal(s) => Ok(Sexp::Literal(s)),
            Identifier(s) => Parser::to_keyword(s),
            EOF => return None,
        };
        Some(expr)
    }

    /// Consume a [`Parser`], returning a list of [`Expression`]'s, or an [`Error`]
    pub fn parse(mut self) -> Result<Vec<Sexp>, Error> {
        std::iter::repeat_with(|| self.parse_expr())
            .take_while(Option::is_some)
            .filter_map(|x| x)
            .collect::<Result<Vec<Sexp>, Error>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use expr::Keyword::*;
    use expr::List;
    use Sexp::*;

    fn cons(car: Sexp, cdr: List) -> List {
        List::Cons(Box::new(car), Box::new(cdr))
    }

    fn id(s: &str) -> Sexp {
        Sexp::Identifier(s.to_string())
    }

    // #[test]
    // fn parse_expr() {
    //     let input = "(lambda (x y) (cons x y))";
    //     let expected = Sexp::List(cons(
    //         Keyword(Lambda), // car
    //         cons(
    //             List(cons(id("x"), cons(id("y"), List::Nil))), // cadr
    //             cons(cons(id("cons"), cons(id("x"), cons(id("y"), List::Nil))), List::Nil)
    //         ),
    //     ));

    //     assert_eq!(Parser::new(input).parse(), Ok(vec![expected]));
    // }

    #[test]
    fn parse_keywords() {
        let input = "(let ((x 0) (y 0))
            (lambda () `(cons ,x y)))";

        let expected = List(cons(
            Keyword(Let),
            cons(
                List(cons(
                    List(cons(id("x"), cons(Integer(0), List::Nil))),
                    cons(List(cons(id("y"), cons(Integer(0), List::Nil))), List::Nil),
                )),
                cons(
                    List(cons(
                        Keyword(Lambda),
                        cons(
                            List(List::Nil),
                            cons(
                                Keyword(Quasiquote),
                                cons(
                                    List(cons(
                                        id("cons"),
                                        cons(
                                            Keyword(Unquote),
                                            cons(id("x"), cons(id("y"), List::Nil)),
                                        ),
                                    )),
                                    List::Nil,
                                ),
                            ),
                        ),
                    )),
                    List::Nil,
                ),
            ),
        ));

        assert_eq!(Parser::new(input).parse(), Ok(vec![expected]));
    }
}
