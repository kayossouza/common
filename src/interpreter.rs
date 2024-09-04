use crate::ast::ASTNode;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(node: ASTNode) {
        match node {
            ASTNode::VariableDeclaration { name, value } => println!("Variable Declaration: {} = {:?}", name, value),
            ASTNode::Number(value) => println!("Number: {}", value),
            ASTNode::Operation(left, right) => {
                println!("Operation:");
                Self::interpret(*left);
                Self::interpret(*right);
            }
            ASTNode::If(condition, then_block, else_block) => {
                // Placeholder: Evaluate 'condition' and execute 'then_block' or 'else_block' accordingly
                println!("If statement: condition = {:?}", condition); 
            }
            ASTNode::GlobalStateDeclaration(name, initial_value) => {
                // Placeholder: Store 'initial_value' in global state with 'name'
                println!("Global state declaration: {} = {:?}", name, initial_value); 
            }
            ASTNode::LocalStateDeclaration(name, initial_value) => {
                // Placeholder: Store 'initial_value' in local state with 'name'
                println!("Local state declaration: {} = {:?}", name, initial_value); 
            }
            ASTNode::StateUpdate(state_name, property, new_value) => {
                // Placeholder: Update the 'property' of 'state_name' with 'new_value'
                println!("State update: {}.{} = {:?}", state_name, property, new_value); 
            }
            ASTNode::ProvideState(state_name, component_or_function_name) => {
                // Placeholder: Make 'state_name' available to 'component_or_function_name'
                println!("Provide state: {} to {}", state_name, component_or_function_name); 
            }
            ASTNode::UseState(state_name, component_or_function_name) => {
                // Placeholder: Access 'state_name' within 'component_or_function_name'
                println!("Use state: {} in {}", state_name, component_or_function_name); 
            }
            ASTNode::String(_) => todo!(),
            ASTNode::Boolean(_) => todo!(),
            ASTNode::Object => todo!(),
            ASTNode::Array => todo!(),
        }
    }
}