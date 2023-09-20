// TODO: Add more AST nodes.
// TODO: Implement spans.
/// The AST nodes for the parser.
///
/// Current formal grammar:
/// <program> ::= <function>
/// <function> ::= "int" <id> "(" ")" "{" <statement> "}"
/// <statement> ::= "return" <exp> ";"
/// <exp> ::= <unary_op> <exp> | <int>
/// <unary_op> ::= "!" | "~" | "-"

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
    Constant(Constant),
    UnaryOp(UnaryOp, Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Constant {
    String(String),
    Int(i32),
    Bool(bool),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Negation,
    LogicalNegation,
    BitwiseComplement,
}
