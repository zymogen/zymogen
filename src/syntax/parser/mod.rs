use super::*;
mod expr;
pub use expr::{Expression, Keyword};

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

    fn parse_list(&mut self) -> Result<Expression, Error> {
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

        // let mut list = Expression::Nil;
        // while let Some(n) = vec.pop() {
        //     list = Expression::List(Box::new(n), Box::new(list));
        // }

        Ok(Expression::List(vec))
    }

    fn to_keyword(ident: String) -> Result<Expression, Error> {
        use expr::Keyword::*;
        use Expression::*;
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
    pub fn parse_expr(&mut self) -> Option<Result<Expression, Error>> {
        use TokenKind::*;
        let token = match self.consume() {
            Err(e) => return Some(Err(e)),
            Ok(token) => token,
        };
        dbg!(&token);
        let expr = match token.kind {
            LeftParen => self.parse_list(),
            RightParen => Err(Error::from_token(&token, ErrorKind::Unbalanced)),
            Quote => Ok(Expression::Keyword(Keyword::Quote)),
            Quasiquote => Ok(Expression::Keyword(Keyword::Quasiquote)),
            Unquote => Ok(Expression::Keyword(Keyword::Unquote)),
            UnquoteAt => Ok(Expression::Keyword(Keyword::UnquoteAt)),
            Dot => Err(Error::from_token(&token, ErrorKind::Unbalanced)),
            Boolean(b) => Ok(Expression::Boolean(b)),
            Integer(i) => Ok(Expression::Integer(i)),
            Literal(s) => Ok(Expression::Literal(s)),
            Identifier(s) => Parser::to_keyword(s),
            EOF => return None,
        };
        Some(expr)
    }

    /// Consume a [`Parser`], returning a list of [`Expression`]'s, or an [`Error`]
    pub fn parse(mut self) -> Result<Vec<Expression>, Error> {
        std::iter::repeat_with(|| self.parse_expr())
            .take_while(Option::is_some)
            .filter_map(|x| x)
            .collect::<Result<Vec<Expression>, Error>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use expr::Keyword::*;
    use Expression::*;

    #[test]
    fn parse_expr() {
        let input = "(lambda (x y) (cons x y))";
        let expected = vec![Expression::List(vec![
            Keyword(Lambda),
            List(vec![
                Identifier("x".to_string()),
                Identifier("y".to_string()),
            ]),
            List(vec![
                Identifier("cons".to_string()),
                Identifier("x".to_string()),
                Identifier("y".to_string()),
            ]),
        ])];
        assert_eq!(Parser::new(input).parse(), Ok(expected));
    }

    #[test]
    fn parse_keywords() {
        let input = "(let ((x 0) (y 0))
            (lambda () `(cons ,x y)))";
        let expected = vec![Expression::List(vec![
            Keyword(Let),
            List(vec![
                List(vec![Identifier("x".to_string()), Integer(0)]),
                List(vec![Identifier("y".to_string()), Integer(0)]),
            ]),
            List(vec![
                Keyword(Lambda),
                List(vec![]),
                Keyword(Quasiquote),
                List(vec![
                    Identifier("cons".to_string()),
                    Keyword(Unquote),
                    Identifier("x".to_string()),
                    Identifier("y".to_string()),
                ]),
            ]),
        ])];

        assert_eq!(Parser::new(input).parse(), Ok(expected));
    }
}
