/// The AST nodes for the parser.
///
/// Current AST definition:
/// program = Program(function_declaration)
/// function_declaration = Function(string, statement) //string is the function name
/// statement = Return(exp)
/// exp = BinOp(binary_operator, exp, exp)
///     | UnOp(unary_operator, exp)
///     | Constant(int)
///
/// Current formal grammar:
/// <program> ::= <function>
/// <function> ::= "int" <id> "(" ")" "{" <statement> "}"
/// <statement> ::= "return" <exp> ";"
/// <exp> ::= <term> { ("+" | "-") <term> }
/// <term> ::= <factor> { ("*" | "/") <factor> }
/// <factor> ::= "(" <exp> ")" | <unary_op> <factor> | <int>
/// <unary_op> ::= "!" | "~" | "-"
///
/// TODO: Add more AST nodes.
/// TODO: Implement spans.

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

#[derive(Debug, PartialEq)]
pub enum Expr {
    Constant(Constant),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Constant {
    String(String),
    Int(i32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Negation,
    LogicalNegation,
    BitwiseComplement,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, PartialEq)]
pub enum OpAssociativity {
    Left,
    Right,
}
