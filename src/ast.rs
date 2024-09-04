pub enum ASTNode {
  Variable(String),
  Number(i64),
  Operation(Box<ASTNode>, Box<ASTNode>),
  If(Box<ASTNode>, Vec<ASTNode>, Option<Vec<ASTNode>>), 
  GlobalStateDeclaration(String, Box<ASTNode>), 
  LocalStateDeclaration(String, Box<ASTNode>),
  StateUpdate(String, String, Box<ASTNode>),
  ProvideState(String, String), 
  UseState(String, String),
}