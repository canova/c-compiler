pub mod ast;

use std::vec;

use crate::lexer::{Keyword, Token, TokenKind, TokenStream};
use ast::*;

pub struct Parser {
    token_stream: TokenStream,
}

impl Parser {
    pub fn new(token_stream: TokenStream) -> Parser {
        Parser { token_stream }
    }

    pub fn parse(mut self) -> Result<Program, String> {
        self.parse_program()
    }
}

impl Parser {
    fn next(&mut self) -> Option<Token> {
        self.token_stream.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.token_stream.tokens.peek()
    }

    fn peek_token_kind(&mut self, expected: TokenKind) -> Result<&Token, String> {
        match self.peek() {
            Some(token) if token.kind == expected => Ok(token),
            Some(other) => Err(format!("Expected token {:?} but got {:?}", expected, other)),
            None => Err(format!("Expected token {:?} but got EOF", expected)),
        }
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, String> {
        match self.next() {
            Some(token) if token.kind == expected => Ok(token),
            Some(token) => Err(format!(
                "Expected token {:?}, but got {:?}",
                expected, token
            )),
            None => Err(format!("Expected token {:?}, but got EOF", expected)),
        }
    }

    fn expect_keyword(&mut self, expected: Keyword) -> Result<Token, String> {
        match self.next() {
            Some(token) if token.kind == TokenKind::Keyword(expected.clone()) => Ok(token),
            Some(token) => Err(format!(
                "Expected keyword {:?}, but got {:?}",
                expected, token
            )),
            None => Err(format!("Expected keyword {:?}, but got EOF", expected)),
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match self.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(ident) => Ok(ident),
                _ => Err(format!("Expected identifier, but got {:?}", token)),
            },
            None => Err("Expected identifier, but got EOF".to_string()),
        }
    }
}

impl Parser {
    fn parse_program(&mut self) -> Result<Program, String> {
        let function = self.parse_function(Some("main"))?;
        Ok(Program { function })
    }

    fn parse_function(&mut self, expected_name: Option<&str>) -> Result<Function, String> {
        self.expect_keyword(Keyword::Int)?;
        let function_name = self.expect_ident()?;

        if let Some(expected_name) = expected_name {
            if function_name != expected_name {
                return Err(format!(
                    "Expected function name {:?}, but got {:?}",
                    expected_name, function_name
                ));
            }
        }

        self.expect(TokenKind::LParen)?;
        // TODO: Implement the arguments.
        self.expect(TokenKind::RParen)?;

        self.expect(TokenKind::LBrace)?;
        let body = self.parse_statements()?;
        self.expect(TokenKind::RBrace)?;

        Ok(Function {
            name: function_name,
            body,
        })
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];
        while let Err(_) = self.peek_token_kind(TokenKind::RBrace) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.next() {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Return) => {
                    let expr = self.parse_expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Statement::Return(Box::new(expr)))
                }
                other => Err(format!("Expected a statement, but got {:?}", other)),
            },
            None => Err("Expected statement, but got EOF".to_string()),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.next() {
            Some(token) => match token.kind {
                // TODO: Implement the other expression types.
                TokenKind::Integer(int) => Ok(Expr::Int(int)),
                other => Err(format!("Expected expression, but got {:?}", other)),
            },
            None => Err("Expected expression, but got EOF".to_string()),
        }
    }
}
