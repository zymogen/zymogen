#[derive(PartialEq, PartialOrd, Debug)]
pub enum ErrorKind {
    EOF,
    Invalid(char),
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub pos: u32,
    pub line: u32,
}
