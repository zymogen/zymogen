mod error;
mod lexer;
mod parser;
pub use error::{Error, ErrorKind};
pub use lexer::{
    token::{Token, TokenKind},
    Lexer,
};

pub fn lex<S: AsRef<str>>(s: S) -> Result<Vec<Token>, String> {
    match Lexer::new(s.as_ref()).lex() {
        Ok(v) => Ok(v),
        Err(e) => {
            let msg = match e.kind {
                ErrorKind::EOF => "unexpected end-of-file",
                ErrorKind::Invalid(_) => "invalid character in input",
            };

            if e.line > 1 {
                let t = e.line as usize;
                let mut output = format!("Error at line {} char {}: {}\n", e.line, e.pos, msg);
                for (ln, line) in s
                    .as_ref()
                    .lines()
                    .enumerate()
                    .skip(e.line as usize - std::cmp::min(e.line, 5) as usize)
                    .take(std::cmp::min(e.line, 5) as usize + 1)
                {
                    output.push_str(&format!("{:>4}|    {}\n", ln, line.trim()));
                    if ln == e.line as usize {
                        output.push_str(&format!(
                            "{}^~~~\n",
                            (0..9 + e.pos).map(|_| ' ').collect::<String>()
                        ));
                    }
                }

                Err(output)
            } else {
                Err(format!(
                    "Error at line {} char {}: {}\n{:>4}|    {}{}^~~_\n",
                    e.line,
                    e.pos,
                    msg,
                    e.line,
                    s.as_ref(),
                    (0..9 + e.pos).map(|_| ' ').collect::<String>()
                ))
            }
        }
    }
}
