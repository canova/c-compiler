/// The AST nodes for the parser.
///
/// Current AST definition:
/// program = Program(function_declaration)
/// function_declaration = Function(string, statement list) //string is the function name
///
/// statement = Return(exp)
///           | Declare(string, exp option) //string is variable name
///                                         //exp is optional initializer
///           | Exp(exp)
///
/// exp = Assign(string, exp)
///     | Var(string) //string is variable name
///     | BinOp(binary_operator, exp, exp)
///     | UnOp(unary_operator, exp)
///     | Constant(int)
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
    VarDecl(VarDecl),
    Return(Box<Expr>),
    Expression(Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct VarDecl {
    pub name: String,
    pub size: VarSize,
    pub initializer: Option<Expr>,
}

impl VarDecl {
    pub fn get_byte_size(&self) -> usize {
        self.size.to_bytes()
    }
}

// TODO: Only Word is supported at the moment, support others.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum VarSize {
    /// 1 byte
    Byte,
    /// 4 bytes in ARM64
    Word,
    /// 8 bytes in ARM64
    DoubleWord,
    /// 16 bytes in ARM64
    QuadWord,
}

impl VarSize {
    pub fn to_bytes(self) -> usize {
        match self {
            VarSize::Byte => 1,
            VarSize::Word => 4,
            VarSize::DoubleWord => 8,
            VarSize::QuadWord => 16,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Assignment(String, Box<Expr>),
    Var(String),
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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEq,
    GreaterThan,
    GreaterThanOrEq,
    Modulo,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

#[derive(Debug, PartialEq)]
pub enum OpAssociativity {
    Left,
    Right,
}

impl BinaryOp {
    pub fn is_short_circuiting_op(&self) -> bool {
        matches!(self, BinaryOp::And | BinaryOp::Or)
    }
}
