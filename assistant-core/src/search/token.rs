#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Term(String),
    Phrase(String),

    And,
    Or,
    Not,

    LParen,
    RParen,

    EOF,
}
