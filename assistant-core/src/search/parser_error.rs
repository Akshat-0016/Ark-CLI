#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,

    UnexpectedToken(String),

    MissingClosingParenthesis,
}
