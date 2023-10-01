/// The AST nodes for the parser.
///
/// Current AST definition:
/// program = Program(function_declaration)
/// function_declaration = Function(string, block_item list) //string is the function name
///
/// block_item = Statement(statement) | Declaration(declaration)
///
/// declaration = Declare(string, exp option) //string is variable name
///                                          //exp is optional initializer
///
/// statement = Return(exp)
///           | Exp(exp)
///           | Conditional(exp, block_item list , block_item list) //exp is controlling condition
///                                                           //first block item is 'if' block
///                                                           //second block item is optional 'else' block
///
/// exp = Assign(string, exp)
///     | Var(string) //string is variable name
///     | BinOp(binary_operator, exp, exp)
///     | UnOp(unary_operator, exp)
///     | Constant(int)
///     | CondExp(exp, exp, exp) //the three expressions are the condition, 'if' expression and 'else' expression, respectively
///
/// TODO: Implement spans.

#[derive(Debug, PartialEq)]
pub struct Program {
    pub function: Function,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub body: Block,
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub items: Vec<BlockItem>,
}

#[derive(Debug, PartialEq)]
pub enum BlockItem {
    Statement(Statement),
    Declaration(VarDecl),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Block(Block),
    Return(Box<Expr>),
    Expression(Box<Expr>),
    Conditional(Conditional),
    While(Box<Expr>, Box<Statement>),   // condition, body
    DoWhile(Box<Statement>, Box<Expr>), // body, condition
    For(For),
    Break,
    Continue,
    Null, // This is not the null keyword. It's a null statement, e.g `;`.
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
    TernaryConditional(TernaryConditional),
    Null, // This is not the null keyword. It's a null expression, e.g `;`.
}

#[derive(Debug, PartialEq)]
pub struct Conditional {
    pub condition: Expr,
    pub if_stmt: Box<Statement>,
    pub else_stmt: Option<Box<Statement>>,
}

#[derive(Debug, PartialEq)]
pub struct TernaryConditional {
    pub condition: Box<Expr>,
    pub if_expr: Box<Expr>,
    pub else_expr: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct For {
    pub init: Box<DeclOrExpr>,
    pub condition: Box<Expr>,
    pub increment: Box<Expr>,
    pub body: Box<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum DeclOrExpr {
    Declaration(VarDecl),
    Expression(Expr),
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
