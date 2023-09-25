use thiserror::Error;

use crate::parser::ast::BinaryOp;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("No function found")]
    NoFunctionFound,

    #[error("Variable {0:?} is already declared")]
    VarAlreadyDeclared(String),
    #[error("Variable {0:?} not found")]
    VarNotFound(String),

    #[error("Unexpected binary operator {0:?}")]
    UnexpectedBinaryOp(BinaryOp),
}
