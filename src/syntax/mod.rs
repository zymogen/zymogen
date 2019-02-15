mod error;
mod lexer;
mod token;
pub use error::{Error, ErrorKind};
pub use lexer::Lexer;
pub use token::{Token, TokenKind};

pub fn lex<S: AsRef<str>>(s: S) -> Result<Vec<Token>, String> {
    match Lexer::new(s.as_ref()).lex() {
        Ok(v) => Ok(v),
        Err(e) => {
            let msg = match e.kind {
                ErrorKind::EOF => "unexpected end-of-file",
                ErrorKind::Invalid(ch) => "invalid character in input",
            };

            if e.line > 2 {
                let t = e.line as usize;
                let lines = s
                    .as_ref()
                    .lines()
                    .skip(e.line as usize - 2)
                    .take(2)
                    .map(String::from)
                    .collect::<Vec<String>>();
                Err(format!(
                    "...\n{}\n{}^~~{}\n",
                    lines.join("\n"),
                    (0..e.pos).map(|_| ' ').collect::<String>(),
                    msg
                ))
            } else {
                Err(format!(
                    "{}{}^~~\n",
                    s.as_ref(),
                    (0..e.pos).map(|_| ' ').collect::<String>()
                ))
            }
        }
    }
}
