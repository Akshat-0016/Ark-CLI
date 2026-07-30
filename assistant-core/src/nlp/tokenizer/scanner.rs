//! Character classification utilities.

pub struct Scanner;

impl Scanner {
    /// Returns true if the character is alphabetic.
    pub fn is_letter(ch: char) -> bool {
        ch.is_alphabetic()
    }

    /// Returns true if the character is numeric.
    pub fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    /// Returns true if the character is whitespace.
    pub fn is_whitespace(ch: char) -> bool {
        ch.is_whitespace()
    }

    /// Returns true if the character is punctuation.
    pub fn is_punctuation(ch: char) -> bool {
        ch.is_ascii_punctuation()
    }

    /// Returns true if the character starts an identifier.
    pub fn is_identifier_start(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    /// Returns true if the character can continue an identifier.
    pub fn is_identifier(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_'
    }

    /// Returns true if the character is a newline.
    pub fn is_newline(ch: char) -> bool {
        ch == '\n'
    }

    /// Returns true if the character is uppercase.
    pub fn is_uppercase(ch: char) -> bool {
        ch.is_uppercase()
    }

    /// Returns true if the character is lowercase.
    pub fn is_lowercase(ch: char) -> bool {
        ch.is_lowercase()
    }
}
