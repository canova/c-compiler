mod helpers;
mod whitespace;

pub mod token;
use self::helpers::*;
use self::token::{Span, Token, TokenKind};

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
    pub pos: usize,
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
        Ok(TokenStream { tokens, pos: 0 })
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
