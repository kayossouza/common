#[derive(Debug)]  // Add this line to implement Debug trait
pub enum ASTNode {
    VariableDeclaration { name: String, value: Box<ASTNode> },
    Number(String),
    String(String),
    Boolean(bool),
    Object,
    Array,
    Operation(Box<ASTNode>, Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>),
    GlobalStateDeclaration(String, Box<ASTNode>),
    LocalStateDeclaration(String, Box<ASTNode>),
    StateUpdate(String, String, Box<ASTNode>),
    ProvideState(String, String),
    UseState(String, String),
    FunctionDeclaration { name: String, parameters: Vec<String>, body: Box<ASTNode> },
}