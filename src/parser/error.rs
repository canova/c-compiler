use thiserror::Error;

use crate::lexer::{Keyword, TokenKind};

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Expected token {0:?} but got {1:?}")]
    UnexpectedToken(TokenKind, TokenKind),
    #[error("Expected token {0:?} but got EOF")]
    UnexpectedEOF(TokenKind),

    #[error("Expected keyword {0:?}, but got {1:?}")]
    UnexpectedTokenForKeyword(Keyword, TokenKind),
    #[error("Expected keyword {0:?}, but got EOF")]
    UnexpectedEOFForKeyword(Keyword),

    #[error("Expected identifier, but got {0:?}")]
    UnexpectedTokenForIdent(TokenKind),
    #[error("Expected identifier, but got EOF")]
    UnexpectedEOFForIdent,

    #[error("Expected function name {0:?}, but got {1:?}")]
    UnexpectedFunctionName(String, String),

    #[error("Expected block item, but got EOF")]
    UnexpectedEOFForBlockItem,
    #[error("Expected statement, but got EOF")]
    UnexpectedEOFForStatement,

    #[error("Expected atom, but got a binary operator {0:?}")]
    UnexpectedBinOpForAtom(TokenKind),
    #[error("Expected atom, but got {0:?}")]
    UnexpectedTokenForAtom(TokenKind),
    #[error("Expected atom, but got EOF")]
    UnexpectedEOFForAtom,

    #[error("Expected unary operator, but got {0:?}")]
    UnexpectedTokenForUnaryOp(TokenKind),
    #[error("Expected binary operator, but got {0:?}")]
    UnexpectedTokenForBinaryOp(TokenKind),
    #[error("Expected operator, but got {0:?}")]
    UnexpectedTokenForOp(TokenKind),
}
