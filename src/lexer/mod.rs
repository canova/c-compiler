mod helpers;
mod whitespace;

pub mod token;
use self::helpers::*;
pub use self::token::*;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Peekable<IntoIter<Token>>,
}

pub struct Tokenizer<'a> {
    remaining_source: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            remaining_source: input,
            pos: 0,
        }
    }

    /// Tokenize the entire input stream and consume the tokenizer.
    pub fn tokenize(mut self) -> Result<TokenStream, String> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(TokenStream {
            tokens: tokens.into_iter().peekable(),
        })
    }

    fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

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
    fn skip_whitespace(&mut self) {
        let skipped = whitespace::skip(self.remaining_source);
        self.chomp(skipped);
    }

    /// Try to lex a single token from the input stream.
    pub fn tokenize_single_token(&mut self) -> Result<TokenKind, String> {
        let data = self.remaining_source;
        let next = match data.chars().next() {
            Some(c) => c,
            None => return Err("Unexpected EOF".into()),
        };

        let (tok, length) = match next {
            '(' => (TokenKind::LParen, 1),
            ')' => (TokenKind::RParen, 1),
            '{' => (TokenKind::LBrace, 1),
            '}' => (TokenKind::RBrace, 1),
            ';' => (TokenKind::Semicolon, 1),
            '0'..='9' => tokenize_integer(data)?,
            c @ '_' | c if c.is_alphabetic() => tokenize_ident(data)?,
            other => return Err(format!("Unknown character: {:?}", other)),
        };

        self.chomp(length);
        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_source = &self.remaining_source[num_bytes..];
        self.pos += num_bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // TODO: Add the other stages here as we go along.
    static ALLOWED_STAGES: &'static [&str] = &["stage_1"];

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

                println!("Testing: {}", path);
                let contents = fs::read_to_string(path).unwrap();
                let tokenizer = Tokenizer::new(&contents);
                let token_stream = tokenizer.tokenize();
                assert!(!token_stream.is_err())
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
}
