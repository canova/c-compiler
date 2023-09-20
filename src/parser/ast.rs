// TODO: Add more AST nodes.
// TODO: Implement spans.

#[derive(Debug, PartialEq)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Return(Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Expr {
    String(String),
    Int(i32),
    Bool(bool),
}
