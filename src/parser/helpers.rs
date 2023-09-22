use crate::lexer::{Token, TokenKind};
use crate::parser::{BinaryOp, Expr, OpAssociativity, UnaryOp};

impl Token {
    #[allow(dead_code)]
    pub fn is_unary_op(&self) -> bool {
        self.kind.is_unary_op()
    }

    pub fn is_binary_op(&self) -> bool {
        self.kind.is_binary_op()
    }

    #[allow(dead_code)]
    pub fn get_unary_op(&self, expr: Expr) -> Result<Expr, String> {
        self.kind.get_unary_op(expr)
    }

    pub fn get_bin_op(&self, lhs: Expr, rhs: Expr) -> Result<Expr, String> {
        self.kind.get_bin_op(lhs, rhs)
    }

    pub fn get_op_pres_assoc(&self) -> Result<(u8, OpAssociativity), String> {
        self.kind.get_op_pres_assoc()
    }
}

impl TokenKind {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash
        )
    }

    pub fn is_unary_op(&self) -> bool {
        matches!(
            self,
            TokenKind::Minus | TokenKind::LogicalNegation | TokenKind::BitwiseComplement
        )
    }

    pub fn get_unary_op(&self, expr: Expr) -> Result<Expr, String> {
        Ok(Expr::UnaryOp(
            match &self {
                TokenKind::Minus => UnaryOp::Negation,
                TokenKind::LogicalNegation => UnaryOp::LogicalNegation,
                TokenKind::BitwiseComplement => UnaryOp::BitwiseComplement,
                other => return Err(format!("Expected unary operator, but got {:?}", other)),
            },
            Box::new(expr),
        ))
    }

    pub fn get_bin_op(&self, lhs: Expr, rhs: Expr) -> Result<Expr, String> {
        Ok(Expr::BinaryOp(
            match &self {
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
        match &self {
            TokenKind::Plus | TokenKind::Minus => Ok((1, OpAssociativity::Left)),
            TokenKind::Asterisk | TokenKind::Slash => Ok((2, OpAssociativity::Left)),
            TokenKind::LogicalNegation | TokenKind::BitwiseComplement => {
                Ok((3, OpAssociativity::Right))
            }
            other => Err(format!("Expected operator, but got {:?}", other)),
        }
    }
}
