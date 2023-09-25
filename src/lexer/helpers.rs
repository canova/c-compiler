use crate::lexer::{
    error::TokenizerError,
    token::{Keyword, TokenKind},
    TokenizerResult,
};

/// Consume bytes while a predicate evaluates to true.
pub fn take_while<F>(data: &str, mut pred: F) -> TokenizerResult<(&str, usize)>
where
    F: FnMut(char) -> bool,
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        Err(TokenizerError::NoMatches)
    } else {
        Ok((&data[..current_index], current_index))
    }
}

/// Consume an identifier from the input stream.
pub fn tokenize_ident(data: &str) -> TokenizerResult<(TokenKind, usize)> {
    // identifiers can't start with a number
    match data.chars().next() {
        Some(ch) if ch.is_ascii_digit() => return Err(TokenizerError::IdentifierStartsWithNumber),
        None => return Err(TokenizerError::UnexpectedEOF),
        _ => {}
    }

    let (got, bytes_read) = take_while(data, |ch| ch == '_' || ch.is_alphanumeric())?;

    // Add the new keywords here.
    match got {
        "int" => Ok((TokenKind::Keyword(Keyword::Int), bytes_read)),
        "return" => Ok((TokenKind::Keyword(Keyword::Return), bytes_read)),
        "if" => Ok((TokenKind::Keyword(Keyword::If), bytes_read)),
        "else" => Ok((TokenKind::Keyword(Keyword::Else), bytes_read)),
        _ => Ok((TokenKind::Identifier(got.to_string()), bytes_read)),
    }
}

/// Tokenize an integer or a float.
pub fn tokenize_integer(data: &str) -> TokenizerResult<(TokenKind, usize)> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = take_while(data, |c| {
        if c.is_ascii_digit() {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    if seen_dot {
        let n: f64 = decimal.parse()?;
        Ok((TokenKind::Decimal(n), bytes_read))
    } else {
        let n: i32 = decimal.parse()?;
        Ok((TokenKind::Integer(n), bytes_read))
    }
}
