// src/lexer.rs

// Enum to represent different types of tokens
// Automatically derive the Debug trait for TokenKind
#[derive(Debug)]
pub enum TokenKind {
  Identifier,
  Number,
  String,
  Symbol,
  EOF, // End of file
}

// Structure to represent a token
// Automatically derive the Debug trait for TokenKind
#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub text: String,
}

impl Token {
  // Constructor to create a new token
  pub fn new(kind: TokenKind, text: String) -> Self {
      Token { kind, text }
  }
}

// src/lexer.rs

pub struct Lexer {
  input: String,
  position: usize,
}

impl Lexer {
  // Constructor to create a new lexer
  pub fn new(input: String) -> Self {
      Lexer { input, position: 0 }
  }

  // Method to get the next character without advancing the position
  fn peek(&self) -> Option<char> {
      self.input.chars().nth(self.position)
  }

  // Method to advance to the next character
  fn advance(&mut self) -> Option<char> {
      let ch = self.peek();
      if ch.is_some() {
          self.position += 1;
      }
      ch
  }

  // Main method to tokenize the input
  pub fn next_token(&mut self) -> Token {
      // Skip whitespace
      while let Some(ch) = self.peek() {
          if ch.is_whitespace() {
              self.advance();
          } else {
              break;
          }
      }

      // Check the current character and return the appropriate token
      let ch = self.advance();

      match ch {
          Some(c) if c.is_alphabetic() => {
              let mut text = c.to_string();
              while let Some(next_ch) = self.peek() {
                  if next_ch.is_alphanumeric() {
                      text.push(next_ch);
                      self.advance();
                  } else {
                      break;
                  }
              }
              Token::new(TokenKind::Identifier, text)
          }
          Some(c) if c.is_digit(10) => {
              let mut text = c.to_string();
              while let Some(next_ch) = self.peek() {
                  if next_ch.is_digit(10) {
                      text.push(next_ch);
                      self.advance();
                  } else {
                      break;
                  }
              }
              Token::new(TokenKind::Number, text)
          }
          Some('"') => {
              let mut text = String::new();
              while let Some(next_ch) = self.advance() {
                  if next_ch == '"' {
                      break;
                  }
                  text.push(next_ch);
              }
              Token::new(TokenKind::String, text)
          }
          Some(c) => {
              let text = c.to_string();
              Token::new(TokenKind::Symbol, text)
          }
          None => Token::new(TokenKind::EOF, String::new()),
      }
  }
}

