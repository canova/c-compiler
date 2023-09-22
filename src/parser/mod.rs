pub mod ast;
mod helpers;

use std::vec;

use crate::lexer::{Keyword, Token, TokenKind, TokenStream};
pub use ast::*;

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
        while self.peek_token_kind(TokenKind::RBrace).is_err() {
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
        self.parse_expr_with_min_precedence(1)
    }

    /// Parse an expression with an operator-precedence parser using precedence
    /// climbing method.
    /// https://en.wikipedia.org/wiki/Operator-precedence_parser#Precedence_climbing_method
    fn parse_expr_with_min_precedence(&mut self, min_precedence: u8) -> Result<Expr, String> {
        let mut atom_lhs = self.parse_atom()?;

        loop {
            if self.peek_token_kind(TokenKind::Semicolon).is_ok() {
                // Break if we see a semicolon. It will be consumed later.
                break;
            }

            let next = self.peek().map(|f| (*f).clone());
            match next {
                None => break,
                Some(ref op) if op.is_binary_op() => {
                    let (precedence, assoc) = op.get_op_prec_assoc()?;
                    if precedence < min_precedence {
                        break;
                    }

                    // Advance the token stream.
                    let _ = self.next();

                    let next_min_precedence = if assoc == OpAssociativity::Left {
                        precedence + 1
                    } else {
                        precedence
                    };
                    let atom_rhs = self.parse_expr_with_min_precedence(next_min_precedence)?;
                    atom_lhs = op.get_bin_op(atom_lhs, atom_rhs)?;
                }
                _ => break,
            }
        }

        Ok(atom_lhs)
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        let token = self.next().ok_or("Expected atom, but got EOF")?;
        match token.kind {
            TokenKind::Integer(int_val) => Ok(Expr::Constant(Constant::Int(int_val))),
            TokenKind::LParen => {
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            // Unary ops
            op if op.is_unary_op() => {
                let expr = self.parse_atom()?;
                op.get_unary_op(expr)
            }
            // Warning for binary ops
            other if other.is_binary_op() => Err(format!(
                "Expected atom, but got a binary operator {:?}",
                other
            )),
            other => Err(format!("Expected atom, but got {:?}", other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{tests::ALLOWED_STAGES, Tokenizer};

    #[test]
    fn test_parser_valid_files() {
        use std::fs;

        let test_dirs = fs::read_dir("tests/").unwrap();
        for dir in test_dirs {
            let dir = dir.unwrap();
            if !dir.file_type().unwrap().is_dir() {
                // Skip the files.
                continue;
            }

            if !ALLOWED_STAGES.contains(&dir.file_name().to_str().unwrap()) {
                // Skip the invalid directory.
                continue;
            }

            let mut path = dir.path();
            path.push("valid");
            let test_files = fs::read_dir(path).unwrap();

            for file in test_files {
                let file = file.unwrap();
                let path = file.path();
                let path = path.to_str().unwrap();

                println!("Testing parser for: {}", path);
                let contents = fs::read_to_string(path).unwrap();
                let tokenizer = Tokenizer::new(&contents);
                let token_stream = tokenizer.tokenize();

                if path.contains("skip_on_failure") && token_stream.is_err() {
                    println!("Failed but skipping: {}", path);
                    continue;
                }

                let parser = Parser::new(token_stream.unwrap());
                let program_ast = parser.parse();
                assert!(!program_ast.is_err());
            }
        }
    }

    #[test]
    fn test_parser_invalid_files() {
        use std::fs;

        let test_dirs = fs::read_dir("tests/").unwrap();
        for dir in test_dirs {
            let dir = dir.unwrap();
            if !dir.file_type().unwrap().is_dir() {
                // Skip the files.
                continue;
            }

            if !ALLOWED_STAGES.contains(&dir.file_name().to_str().unwrap()) {
                // Skip the invalid directory.
                continue;
            }

            let mut path = dir.path();
            path.push("invalid");
            let test_files = fs::read_dir(path).unwrap();

            for file in test_files {
                let file = file.unwrap();
                let path = file.path();
                let path = path.to_str().unwrap();

                println!("Testing parser for: {}", path);
                let contents = fs::read_to_string(path).unwrap();
                let tokenizer = Tokenizer::new(&contents);
                let token_stream = tokenizer.tokenize().unwrap();

                let parser = Parser::new(token_stream);
                let program_ast = parser.parse();
                assert!(program_ast.is_err());
            }
        }
    }
}
