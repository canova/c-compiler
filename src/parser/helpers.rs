use crate::lexer::{Token, TokenKind};
use crate::parser::{BinaryOp, Expr, OpAssociativity};

impl Token {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash
        )
    }

    pub fn get_bin_op(&self, lhs: Expr, rhs: Expr) -> Result<Expr, String> {
        Ok(Expr::BinaryOp(
            match &self.kind {
                TokenKind::Plus => BinaryOp::Addition,
                TokenKind::Minus => BinaryOp::Subtraction,
                TokenKind::Asterisk => BinaryOp::Multiplication,
                TokenKind::Slash => BinaryOp::Division,
                other => return Err(format!("Expected binary operator, but got {:?}", other)),
            },
            Box::new(lhs),
            Box::new(rhs),
        ))
    }

    pub fn get_op_pres_assoc(&self) -> Result<(u8, OpAssociativity), String> {
        match &self.kind {
            TokenKind::Plus | TokenKind::Minus => Ok((1, OpAssociativity::Left)),
            TokenKind::Asterisk | TokenKind::Slash => Ok((2, OpAssociativity::Left)),
            TokenKind::LogicalNegation | TokenKind::BitwiseComplement => {
                Ok((3, OpAssociativity::Right))
            }
            other => Err(format!("Expected operator, but got {:?}", other)),
        }
    }
}
