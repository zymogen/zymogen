use super::{Token, TokenKind};
use colored::*;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum ErrorKind {
    EOF,
    Invalid(char),
    ExpectedToken(TokenKind, TokenKind),
    Unbalanced,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: u32,
    pub line: u32,
}

impl Error {
    pub fn message<S: AsRef<str>>(&self, source: S) -> String {
        let msg = match &self.kind {
            ErrorKind::EOF => format!("unexpected end-of-file!"),
            ErrorKind::Invalid(ch) => format!("invalid character in input `{}`!", ch),
            ErrorKind::ExpectedToken(exp, got) => {
                format!("expected TokenKind `{:?}`, found `{:?}!`", exp, got)
            }
            ErrorKind::Unbalanced => format!("unbalanced expression!"),
        };

        let mut output = format!("\nError at line {} char {}\n", self.line, self.pos)
            .yellow()
            .to_string();
        for (ln, line) in source
            .as_ref()
            .lines()
            .enumerate()
            .skip(self.line as usize - std::cmp::min(self.line, 5) as usize)
            .take(std::cmp::min(self.line, 5) as usize + 1)
        {
            output.push_str(&format!("{:>4}|    {}\n", ln, line));
            if ln == self.line as usize {
                output.push_str(&format!(
                    "{}",
                    (0..9 + self.pos).map(|_| ' ').collect::<String>()
                ));
                output.push_str(&format!("^~~~ {}\n", msg).red().to_string());
            }
        }
        output
    }

    pub fn from_token(token: &Token, kind: ErrorKind) -> Error {
        Error {
            kind,
            pos: token.pos - token.kind.size().min(token.pos as usize) as u32,
            line: token.line,
        }
    }
}
