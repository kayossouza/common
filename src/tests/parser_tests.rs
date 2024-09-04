#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_parse_identifier() {
        let input = "test 123";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse(); // Assuming parse will consume tokens correctly
    }
}
