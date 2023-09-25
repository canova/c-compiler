#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Option<Span>,
}

// This is only a subset of the token kinds right now. I'm expanding this as we go along.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Integer(i32),
    Decimal(f64),
    Keyword(Keyword),
    LParen,            // (
    RParen,            // )
    LBrace,            // {
    RBrace,            // }
    Semicolon,         // ;
    Plus,              // +
    Minus,             // -
    Asterisk,          // *
    Slash,             // /
    BitwiseComplement, // ~
    LogicalNegation,   // !
    And,               // &&
    Or,                // ||
    Equal,             // ==
    NotEqual,          // !=
    LessThan,          // <
    LessThanOrEq,      // <=
    GreaterThan,       // >
    GreaterThanOrEq,   // >=
    Modulo,            // %
    BitwiseAnd,        // &
    BitwiseOr,         // |
    BitwiseXor,        // ^
    BitwiseShiftLeft,  // <<
    BitwiseShiftRight, // >>
    Assignment,        // =
    Colon,             // :
    QuestionMark,      // ?
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Return,
    Int,
    If,
    Else,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}
