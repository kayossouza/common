use crate::ast::ASTNode;
use crate::lexer::{Lexer, Token, ComparisonOp}; // Assuming you have defined ComparisonOp in your lexer

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EOF,
        };
        parser.next_token();
        parser
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.parse_program()
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token == expected {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.match_token(expected) {
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, found {:?}",
                expected, self.current_token
            ))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        if let Token::Identifier(ident) = self.current_token {
            let ident_str = ident.to_string();
            self.next_token();
            Ok(ident_str)
        } else {
            Err(format!(
                "Expected identifier, found {:?}",
                self.current_token
            ))
        }
    }

    // --- Parsing Functions ---

    fn parse_program(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        Ok(ASTNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        if self.match_token(Token::Keyword("create")) {
            self.parse_function_declaration()
        } else if let Some(var_type) = self.parse_variable_declaration_start() {
            self.parse_variable_declaration(var_type)
        } else if self.match_token(Token::Keyword("when")) {
            self.parse_if_statement()
        } else if self.match_token(Token::Keyword("do")) {
            self.parse_loop()
        } else if self.match_token(Token::Keyword("while")) {
            self.parse_while_loop()
        } else if self.match_token(Token::Symbol('$')) {
            self.parse_global_state_declaration()
        } else if self.match_token(Token::Symbol('#')) {
            self.parse_local_state_declaration()
        } else if self.match_token(Token::Keyword("update")) {
            self.parse_state_update()
        } else if self.match_token(Token::Keyword("provide")) {
            self.parse_provide_state()
        } else if self.match_token(Token::Keyword("use")) {
            self.parse_use_state()
        } else if let Some(ident) = self.parse_assignment_target() {
            self.parse_assignment(ident)
        } else if let Some(ident) = self.parse_function_call_start() {
            self.parse_function_call(ident)
        } else {
            Err("Unknown statement".to_string())
        }
    }

    fn parse_statement_list(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut statements = Vec::new();
        while !self.match_token(Token::Keyword("end")) && self.current_token != Token::EOF {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Keyword("a"))?;
        self.expect_token(Token::Keyword("function"))?;
        self.expect_token(Token::Keyword("called"))?;

        let name = self.expect_identifier()?;

        let params = if self.match_token(Token::Keyword("with")) {
            self.parse_parameter_list()?
        } else {
            Vec::new()
        };

        self.expect_token(Token::NewLine)?; 

        let body = self.parse_statement_list()?;

        Ok(ASTNode::FunctionDeclaration(name, params, body))
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<String>, String> {
        let mut params = Vec::new();
        params.push(self.expect_identifier()?);

        while self.match_token(Token::Keyword("and")) {
            params.push(self.expect_identifier()?);
        }

        Ok(params)
    }

    fn parse_variable_declaration_start(&mut self) -> Option<VarType> {
        if self.match_token(Token::Keyword("set")) {
            if self.match_token(Token::Keyword("global")) {
                Some(VarType::Global)
            } else if self.match_token(Token::Keyword("local")) {
                Some(VarType::Local)
            } else {
                None 
            }
        } else {
            None
        }
    }

    fn parse_variable_declaration(&mut self, var_type: VarType) -> Result<ASTNode, String> {
        let identifier = self.expect_identifier()?;

        self.expect_token(Token::Keyword("to"))?;

        let value = self.parse_expression()?;

        let is_constant = self.match_token(Token::Keyword("constant"));

        match var_type {
            VarType::Global => Ok(ASTNode::GlobalVariableDeclaration(identifier, Box::new(value), is_constant)),
            VarType::Local => Ok(ASTNode::LocalVariableDeclaration(identifier, Box::new(value), is_constant)),
        }
    }

    fn parse_assignment_target(&mut self) -> Option<String> {
        if let Token::Identifier(ident) = self.current_token {
            let ident_str = ident.to_string();
            self.next_token();
            Some(ident_str)
        } else {
            None
        }
    }

    fn parse_assignment(&mut self, identifier: String) -> Result<ASTNode, String> {
        if self.match_token(Token::Keyword("is")) {
            let value = self.parse_expression()?;
            Ok(ASTNode::Assignment(identifier, Box::new(value)))
        } else if self.match_token(Token::Keyword("change")) {
            self.expect_token(Token::Keyword("to"))?;
            let new_value = self.parse_expression()?;
            Ok(ASTNode::Reassignment(identifier, Box::new(new_value)))
        } else {
            Err(format!(
                "Expected 'is' or 'change to' for assignment, found {:?}",
                self.current_token
            ))
        }
    }

    fn parse_if_statement(&mut self) -> Result<ASTNode, String> {
        let condition = self.parse_expression()?;

        self.expect_token(Token::Keyword("then"))?;
        self.expect_token(Token::NewLine)?;

        let then_block = self.parse_statement_list()?;

        let else_block = if self.match_token(Token::Keyword("else")) {
            self.expect_token(Token::NewLine)?;
            Some(self.parse_statement_list()?)
        } else {
            None
        };

        self.expect_token(Token::Keyword("end"))?; 

        Ok(ASTNode::If(Box::new(condition), then_block, else_block))
    }

    fn parse_loop(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Keyword("this"))?;
        let iterations = if let Token::Number(num_str) = self.current_token {
            let num_str = num_str.to_string();
            self.next_token();
            num_str.parse::<usize>().map_err(|_| "Invalid number of iterations".to_string())?
        } else {
            return Err("Expected a number after 'do this'".to_string());
        };
        self.expect_token(Token::Keyword("times"))?;
        self.expect_token(Token::NewLine)?; 

        let body = self.parse_statement_list()?;

        self.expect_token(Token::Keyword("end"))?; 

        Ok(ASTNode::Loop(iterations, body))
    }

    fn parse_while_loop(&mut self) -> Result<ASTNode, String> {
        let condition = self.parse_expression()?;

        self.expect_token(Token::NewLine)?; 

        let body = self.parse_statement_list()?;

        self.expect_token(Token::Keyword("end"))?; 

        Ok(ASTNode::WhileLoop(Box::new(condition), body))
    }

    fn parse_global_state_declaration(&mut self) -> Result<ASTNode, String> {
        let identifier = self.expect_identifier()?;

        self.expect_token(Token::Keyword("is"))?;

        let initial_value = self.parse_expression()?;

        Ok(ASTNode::GlobalStateDeclaration(
            identifier,
            Box::new(initial_value),
        ))
    }

    fn parse_local_state_declaration(&mut self) -> Result<ASTNode, String> {
        let identifier = self.expect_identifier()?;

        self.expect_token(Token::Keyword("is"))?;

        let initial_value = self.parse_expression()?;

        Ok(ASTNode::LocalStateDeclaration(
            identifier,
            Box::new(initial_value),
        ))
    }

    fn parse_state_update(&mut self) -> Result<ASTNode, String> {
        let state_name = if self.match_token(Token::Symbol('$')) {
            self.expect_identifier()?
        } else if self.match_token(Token::Symbol('#')) {
            self.expect_identifier()?
        } else {
            return Err("Expected '$' or '#' for state update".to_string());
        };

        let property = self.expect_identifier()?;

        self.expect_token(Token::Keyword("to"))?;

        let new_value = self.parse_expression()?;

        Ok(ASTNode::StateUpdate(
            state_name,
            property,
            Box::new(new_value),
        ))
    }

    fn parse_provide_state(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Symbol('$'))?;

        let state_name = self.expect_identifier()?;

        self.expect_token(Token::Keyword("to"))?;

        let component_or_function_name = self.expect_identifier()?;

        Ok(ASTNode::ProvideState(
            state_name,
            component_or_function_name,
        ))
    }

    fn parse_use_state(&mut self) -> Result<ASTNode, String> {
        self.expect_token(Token::Symbol('$'))?;

        let state_name = self.expect_identifier()?;

        self.expect_token(Token::Keyword("in"))?;

        let component_or_function_name = self.expect_identifier()?;

        Ok(ASTNode::UseState(state_name, component_or_function_name))
    }

    fn parse_function_call_start(&mut self) -> Option<String> {
        if let Token::Identifier(ident) = self.current_token {
            let ident_str = ident.to_string();
            self.next_token();
            Some(ident_str)
        } else {
            None
        }
    }

    fn parse_function_call(&mut self, name: String) -> Result<ASTNode, String> {
        let args = if self.current_token != Token::NewLine {
            self.parse_argument_list()?
        } else {
            Vec::new()
        };

        self.expect_token(Token::NewLine)?; 

        Ok(ASTNode::FunctionCall(name, args))
    }

    fn parse_argument_list(&mut self) -> Result<Vec<ASTNode>, String> {
        let mut args = Vec::new();
        args.push(self.parse_expression()?);

        while self.match_token(Token::Keyword("and")) {
            args.push(self.parse_expression()?);
        }

        Ok(args)
    }

    fn parse_comparison_expression(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_additive_expression()?;

        while let Some(op) = self.parse_comparison_op() {
            let right = self.parse_additive_expression()?;
            left = ASTNode::Comparison(op, Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_comparison_op(&mut self) -> Option<ComparisonOp> {
        match self.current_token {
            Token::GreaterThan => {
                self.next_token();
                Some(ComparisonOp::GreaterThan)
            },
            Token::LessThan => {
                self.next_token();
                Some(ComparisonOp::LessThan)
            },
            Token::EqualTo => {
                self.next_token();
                Some(ComparisonOp::EqualTo)
            },
            Token::NotEqualTo => {
                self.next_token();
                Some(ComparisonOp::NotEqualTo)
            },
            Token::GreaterOrEqual => {
                self.next_token();
                Some(ComparisonOp::GreaterOrEqual)
            },
            Token::LessOrEqual => {
                self.next_token();
                Some(ComparisonOp::LessOrEqual)
            },
            _ => None
        }
    }


    fn parse_additive_expression(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.match_token(Token::Plus) || self.match_token(Token::Minus) {
            let operator = self.current_token.clone();
            self.next_token();
            let right = self.parse_multiplicative_expression()?;
            left = ASTNode::Operation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> Result<ASTNode, String> {
        let mut left = self.parse_primary()?;

        while self.match_token(Token::Multiply) || self.match_token(Token::Divide) {
            let operator = self.current_token.clone();
            self.next_token();
            let right = self.parse_primary()?;
            left = ASTNode::Operation(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
      self.parse_logical_expression() 
  }

  fn parse_object_literal(&mut self) -> Result<ASTNode, String> {
      self.expect_token(Token::LBrace)?;

      let mut properties = Vec::new();
      while !self.match_token(Token::RBrace) {
          let key = self.expect_identifier()?;
          self.expect_token(Token::Colon)?;
          let value = self.parse_expression()?;
          properties.push((key, value));

          if !self.match_token(Token::Comma) {
              break; 
          }
      }

      Ok(ASTNode::ObjectLiteral(properties))
  }

  fn parse_array_literal(&mut self) -> Result<ASTNode, String> {
      self.expect_token(Token::LBracket)?;

      let mut elements = Vec::new();
      while !self.match_token(Token::RBracket) {
          elements.push(self.parse_expression()?);

          if !self.match_token(Token::Comma) {
              break;
          }
      }

      Ok(ASTNode::ArrayLiteral(elements))
  }

  fn parse_primary(&mut self) -> Result<ASTNode, String> {
      match self.current_token {
          Token::Identifier(ident) => {
              let ident_str = ident.to_string();
              self.next_token();
              Ok(ASTNode::Variable(ident_str))
          }
          Token::Number(num_str) => {
              let num_str = num_str.to_string();
              self.next_token();
              num_str
                  .parse::<i64>()
                  .map(ASTNode::Number)
                  .map_err(|_| "Invalid number literal".to_string())
          }
          Token::StringLiteral(literal) => {
              let literal_str = literal.to_string();
              self.next_token();
              Ok(ASTNode::StringLiteral(literal_str))
          }
          Token::Keyword("true") => {
              self.next_token();
              Ok(ASTNode::Boolean(true))
          }
          Token::Keyword("false") => {
              self.next_token();
              Ok(ASTNode::Boolean(false))
          }
          Token::LParen => {
              self.next_token();
              let expr = self.parse_expression()?;
              self.expect_token(Token::RParen)?;
              Ok(expr)
          }
          Token::LBrace => self.parse_object_literal(),
          Token::LBracket => self.parse_array_literal(),
          _ => Err(format!("Unexpected token: {:?}", self.current_token)),
      }
  }
}
fn parse_logical_expression(&mut self) -> Result<ASTNode, String> {
  let mut left = self.parse_comparison_expression()?;

  while self.match_token(Token::Keyword("and")) || self.match_token(Token::Keyword("or")) {
      let operator = self.current_token.clone();
      self.next_token();
      let right = self.parse_comparison_expression()?;
      left = ASTNode::Logical(operator, Box::new(left), Box::new(right));
  }

  Ok(left)
}

fn parse_for_each_loop(&mut self) -> Result<ASTNode, String> {
  self.expect_token(Token::Keyword("for"))?;
  self.expect_token(Token::Keyword("every"))?;

  let item_name = self.expect_identifier()?;
  self.expect_token(Token::Keyword("in"))?;
  let collection_name = self.expect_identifier()?;
  self.expect_token(Token::NewLine)?;

  let body = self.parse_statement_list()?;

  self.expect_token(Token::Keyword("end"))?;

  Ok(ASTNode::ForEachLoop(item_name, collection_name, body))
}

enum VarType {
    Global,
    Local,
}