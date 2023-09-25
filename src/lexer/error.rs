use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Unknown character: '{0}'")]
    UnknownCharacter(char),

    #[error("No matches")]
    NoMatches,

    #[error("Identifiers can't start with a number")]
    IdentifierStartsWithNumber,

    #[error("Unexpected EOF")]
    UnexpectedEOF,
}
