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

    pub fn get_op_prec_assoc(&self) -> Result<(u8, OpAssociativity), String> {
        self.kind.get_op_prec_assoc()
    }
}

impl TokenKind {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Asterisk
                | TokenKind::Slash
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::LessThanOrEq
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanOrEq
                | TokenKind::Modulo
                | TokenKind::BitwiseAnd
                | TokenKind::BitwiseOr
                | TokenKind::BitwiseXor
                | TokenKind::BitwiseShiftLeft
                | TokenKind::BitwiseShiftRight
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
                TokenKind::And => BinaryOp::And,
                TokenKind::Or => BinaryOp::Or,
                TokenKind::Equal => BinaryOp::Equal,
                TokenKind::NotEqual => BinaryOp::NotEqual,
                TokenKind::LessThan => BinaryOp::LessThan,
                TokenKind::LessThanOrEq => BinaryOp::LessThanOrEq,
                TokenKind::GreaterThan => BinaryOp::GreaterThan,
                TokenKind::GreaterThanOrEq => BinaryOp::GreaterThanOrEq,
                TokenKind::Modulo => BinaryOp::Modulo,
                TokenKind::BitwiseAnd => BinaryOp::BitwiseAnd,
                TokenKind::BitwiseOr => BinaryOp::BitwiseOr,
                TokenKind::BitwiseXor => BinaryOp::BitwiseXor,
                TokenKind::BitwiseShiftLeft => BinaryOp::BitwiseShiftLeft,
                TokenKind::BitwiseShiftRight => BinaryOp::BitwiseShiftRight,
                other => return Err(format!("Expected binary operator, but got {:?}", other)),
            },
            Box::new(lhs),
            Box::new(rhs),
        ))
    }

    /// https://en.cppreference.com/w/c/language/operator_precedence
    /// TODO: Move this to a static constant.
    pub fn get_op_prec_assoc(&self) -> Result<(u8, OpAssociativity), String> {
        match &self {
            TokenKind::Or => Ok((1, OpAssociativity::Left)),
            TokenKind::And => Ok((2, OpAssociativity::Left)),
            TokenKind::BitwiseOr => Ok((3, OpAssociativity::Left)),
            TokenKind::BitwiseXor => Ok((4, OpAssociativity::Left)),
            TokenKind::BitwiseAnd => Ok((5, OpAssociativity::Left)),
            TokenKind::Equal | TokenKind::NotEqual => Ok((6, OpAssociativity::Left)),
            TokenKind::LessThan
            | TokenKind::LessThanOrEq
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEq => Ok((7, OpAssociativity::Left)),
            TokenKind::BitwiseShiftLeft | TokenKind::BitwiseShiftRight => {
                Ok((8, OpAssociativity::Left))
            }
            TokenKind::Plus | TokenKind::Minus => Ok((9, OpAssociativity::Left)),
            TokenKind::Asterisk | TokenKind::Slash | TokenKind::Modulo => {
                Ok((10, OpAssociativity::Left))
            }
            TokenKind::LogicalNegation | TokenKind::BitwiseComplement => {
                Ok((11, OpAssociativity::Right))
            }
            other => Err(format!("Expected operator, but got {:?}", other)),
        }
    }
}
