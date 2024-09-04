mod lexer;
mod parser;
mod interpreter;
mod ast;

use crate::lexer::{Lexer, TokenKind};

fn main() {
    let input = "let x = 42;".to_string();
    let mut lexer = Lexer::new(input);

    loop {
        let token = lexer.next_token();
        match token.kind {
            TokenKind::EOF => break,
            _ => println!("Token: {:?}, Text: {}", token.kind, token.text),
        }
    }
}
