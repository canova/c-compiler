use crate::tokenizer::{helpers::take_while, TokenizerError, TokenizerResult};

/// Skip past any whitespace characters or comments.
pub fn skip(src: &str) -> TokenizerResult<usize> {
    let mut remaining = src;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let comments = skip_comments(remaining)?;
        remaining = &remaining[comments..];

        if ws + comments == 0 {
            return Ok(src.len() - remaining.len());
        }
    }
}

fn skip_whitespace(data: &str) -> usize {
    match take_while(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

fn skip_comments(src: &str) -> TokenizerResult<usize> {
    let pairs = [("//", "\n"), ("/*", "*/")];

    for &(pattern, matcher) in &pairs {
        if src.starts_with(pattern) {
            let leftovers = skip_until(src, matcher)?;
            return Ok(src.len() - leftovers.len());
        }
    }

    Ok(0)
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> TokenizerResult<&'a str> {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src
            .chars()
            .next()
            .ok_or(TokenizerError::UnexpectedEOF)?
            .len_utf8();
        src = &src[next_char_size..];
    }

    Ok(&src[pattern.len()..])
}

macro_rules! comment_test {
    ($name:ident, $src:expr => $should_be:expr) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let got = skip_comments($src).unwrap();
            assert_eq!(got, $should_be);
        }
    };
}

comment_test!(test_slash_slash_skips_to_end_of_line, "// testing { hello }\n 1234" => 21);
comment_test!(test_comment_skip_curly_braces, "/* test \n 1234 */ hello wor\nld" => 17);
comment_test!(test_comment_skip_ignores_alphanumeric, "123 hello world" => 0);
