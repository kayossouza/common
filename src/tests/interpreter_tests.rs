#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;
    use crate::interpreter::Interpreter;

    #[test]
    fn test_interpreter() {
        let node = ASTNode::Variable("test".to_string());
        Interpreter::interpret(node); // Should print "Variable: test"
    }
}
