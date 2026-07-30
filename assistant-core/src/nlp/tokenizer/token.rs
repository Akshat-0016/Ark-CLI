//! Token definitions used throughout the NLP engine.

use std::fmt;

/// The category of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Word,
    Number,
    Punctuation,
    Symbol,
    Whitespace,
    Newline,
    Unknown,
    EndOfFile,
}

/// A single token extracted from text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    /// Original text of the token.
    lexeme: String,

    /// Category of the token.
    kind: TokenKind,

    /// Byte position where the token starts.
    start: usize,

    /// Byte position where the token ends.
    end: usize,
}

impl Token {
    /// Creates a new token.
    pub fn new(lexeme: String, kind: TokenKind, start: usize, end: usize) -> Self {
        Self {
            lexeme,
            kind,
            start,
            end,
        }
    }

    /// Returns the token text.
    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    /// Returns the token category.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns the start byte position.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end byte position.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the token length in bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns true if the token is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Lower case the letters
    pub fn lexeme_mut(&mut self) -> &mut String {
        &mut self.lexeme
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{:?}] ({}..{})",
            self.lexeme, self.kind, self.start, self.end
        )
    }
}
