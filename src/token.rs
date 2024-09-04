#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    Number(&'a str),
    Unknown(char),
    EOF,
}
