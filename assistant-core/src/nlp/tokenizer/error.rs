use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Invalid character")]
    InvalidCharacter,
}
