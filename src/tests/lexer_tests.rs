#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn test_lex_identifier() {
        let input = "test";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Identifier("test"));
    }

    #[test]
    fn test_lex_number() {
        let input = "123";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Number("123"));
    }
}
