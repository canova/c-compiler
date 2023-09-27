mod error;
mod helpers;
mod whitespace;

pub mod token;
use self::error::TokenizerError;
use self::helpers::*;
pub use self::token::*;
use std::iter::Peekable;
use std::vec::IntoIter;

type TokenizerResult<T> = Result<T, TokenizerError>;

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Peekable<IntoIter<Token>>,
}

pub struct Tokenizer<'a> {
    remaining_source: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    /// Create a new tokenizer from the given input.
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            remaining_source: input,
            pos: 0,
        }
    }

    /// Tokenize the entire input stream and consume the tokenizer.
    pub fn tokenize(mut self) -> TokenizerResult<TokenStream> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(TokenStream {
            tokens: tokens.into_iter().peekable(),
        })
    }

    /// Get the next token from the input stream.
    fn next_token(&mut self) -> TokenizerResult<Option<Token>> {
        self.skip_whitespace()?;

        if self.remaining_source.is_empty() {
            Ok(None)
        } else {
            let start = self.pos;
            let token = self.tokenize_single_token()?;
            let end = self.pos;
            Ok(Some(Token {
                kind: token,
                span: Some(Span { lo: start, hi: end }),
            }))
        }
    }

    /// Skip all the whitespace and comments.
    fn skip_whitespace(&mut self) -> TokenizerResult<()> {
        let skipped = whitespace::skip(self.remaining_source)?;
        self.chomp(skipped);
        Ok(())
    }

    /// Try to lex a single token from the input stream.
    pub fn tokenize_single_token(&mut self) -> TokenizerResult<TokenKind> {
        let data = self.remaining_source;
        let mut rem_chars = data.chars().peekable();
        let next = rem_chars.next().ok_or(TokenizerError::UnexpectedEOF)?;

        let (tok, length) = match next {
            '(' => (TokenKind::LParen, 1),
            ')' => (TokenKind::RParen, 1),
            '{' => (TokenKind::LBrace, 1),
            '}' => (TokenKind::RBrace, 1),
            ';' => (TokenKind::Semicolon, 1),
            '+' => (TokenKind::Plus, 1),
            '-' => (TokenKind::Minus, 1),
            '*' => (TokenKind::Asterisk, 1),
            '/' => (TokenKind::Slash, 1),
            '%' => (TokenKind::Modulo, 1),
            '~' => (TokenKind::BitwiseComplement, 1),
            '^' => (TokenKind::BitwiseXor, 1),
            ':' => (TokenKind::Colon, 1),
            '?' => (TokenKind::QuestionMark, 1),
            '&' if rem_chars.peek() == Some(&'&') => (TokenKind::And, 2),
            '|' if rem_chars.peek() == Some(&'|') => (TokenKind::Or, 2),
            '=' if rem_chars.peek() == Some(&'=') => (TokenKind::Equal, 2),
            '!' if rem_chars.peek() == Some(&'=') => (TokenKind::NotEqual, 2),
            '<' if rem_chars.peek() == Some(&'=') => (TokenKind::LessThanOrEq, 2),
            '>' if rem_chars.peek() == Some(&'=') => (TokenKind::GreaterThanOrEq, 2),
            '<' if rem_chars.peek() == Some(&'<') => (TokenKind::BitwiseShiftLeft, 2),
            '>' if rem_chars.peek() == Some(&'>') => (TokenKind::BitwiseShiftRight, 2),
            // They have to stay after their two-char counterparts.
            '&' => (TokenKind::BitwiseAnd, 1),
            '|' => (TokenKind::BitwiseOr, 1),
            '!' => (TokenKind::LogicalNegation, 1),
            '<' => (TokenKind::LessThan, 1),
            '>' => (TokenKind::GreaterThan, 1),
            '=' => (TokenKind::Assignment, 1),
            '0'..='9' => tokenize_integer(data)?,
            c @ '_' | c if c.is_alphabetic() => tokenize_ident_or_keyword(data)?,
            other => return Err(TokenizerError::UnknownCharacter(other)),
        };

        self.chomp(length);
        Ok(tok)
    }

    /// Consume the given number of bytes from the input stream.
    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_source = &self.remaining_source[num_bytes..];
        self.pos += num_bytes;
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    // Add the other stages here as we go along.
    pub static ALLOWED_STAGES: &'static [&str] =
        &["stage_1", "stage_2", "stage_3", "stage_4", "stage_5"];

    #[test]
    fn test_tokenizer_valid_files() {
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

                println!("Testing lexer for: {}", path);
                let contents = fs::read_to_string(path).unwrap();
                let tokenizer = Tokenizer::new(&contents);
                let token_stream = tokenizer.tokenize();
                if path.contains("skip_on_failure") && token_stream.is_err() {
                    println!("Failed but skipping: {}", path);
                    continue;
                }
                assert!(!token_stream.is_err());
            }
        }
    }

    macro_rules! tokenizer_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let tokenizer = Tokenizer::new($src);
                let token_stream = tokenizer.tokenize().unwrap();
                assert_eq!(token_stream.tokens.len(), $should_be);
            }
        };
    }

    macro_rules! tokenizer_single_token_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[test]
            fn $name() {
                let tokenizer = Tokenizer::new($src);
                let mut token_stream = tokenizer.tokenize().unwrap();
                assert_eq!(token_stream.tokens.len(), 1);
                assert_eq!(token_stream.tokens.next().unwrap(), $should_be);
            }
        };
    }

    tokenizer_test!(test_tokenize_main, "int main() {}" => 6);
    tokenizer_test!(test_tokenize_main_with_return, "int main() { return 0; }" => 9);
    tokenizer_single_token_test!(test_tokenize_int, "int" => Token {
        kind: TokenKind::Keyword(Keyword::Int),
        span: Some(Span { lo: 0, hi: 3 }),
    });
    tokenizer_single_token_test!(test_tokenize_return, "return" => Token {
        kind: TokenKind::Keyword(Keyword::Return),
        span: Some(Span { lo: 0, hi: 6 }),
    });
    tokenizer_single_token_test!(test_tokenize_ident, "testing" => Token {
        kind: TokenKind::Identifier("testing".into()),
        span: Some(Span { lo: 0, hi: 7 }),
    });
    tokenizer_single_token_test!(test_tokenize_integer, "123" => Token {
        kind: TokenKind::Integer(123),
        span: Some(Span { lo: 0, hi: 3 }),
    });
    tokenizer_single_token_test!(test_tokenize_decimal, "123.23" => Token {
        kind: TokenKind::Decimal(123.23),
        span: Some(Span { lo: 0, hi: 6 }),
    });
    tokenizer_single_token_test!(test_tokenize_l_brace, "{" => Token {
        kind: TokenKind::LBrace,
        span: Some(Span { lo: 0, hi: 1 }),
    });
    tokenizer_single_token_test!(test_tokenize_not_equal, "!=" => Token {
        kind: TokenKind::NotEqual,
        span: Some(Span { lo: 0, hi: 2 }),
    });
    tokenizer_single_token_test!(test_tokenize_logical_negation, "!" => Token {
        kind: TokenKind::LogicalNegation,
        span: Some(Span { lo: 0, hi: 1 }),
    });
    tokenizer_single_token_test!(test_tokenize_equal, "==" => Token {
        kind: TokenKind::Equal,
        span: Some(Span { lo: 0, hi: 2 }),
    });
    tokenizer_single_token_test!(test_tokenize_greater_than, ">" => Token {
        kind: TokenKind::GreaterThan,
        span: Some(Span { lo: 0, hi: 1 }),
    });
    tokenizer_single_token_test!(test_tokenize_greater_than_or_eq, ">=" => Token {
        kind: TokenKind::GreaterThanOrEq,
        span: Some(Span { lo: 0, hi: 2 }),
    });
    tokenizer_single_token_test!(test_tokenize_if, "if" => Token {
        kind: TokenKind::Keyword(Keyword::If),
        span: Some(Span { lo: 0, hi: 2 }),
    });
    tokenizer_single_token_test!(test_tokenize_else, "else" => Token {
        kind: TokenKind::Keyword(Keyword::Else),
        span: Some(Span { lo: 0, hi: 4 }),
    });
}
