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

    #[error("Failed to parse a floating point number")]
    FloatParseFailed(#[from] std::num::ParseFloatError),

    #[error("Failed to parse an int number")]
    IntParseFailed(#[from] std::num::ParseIntError),
}
