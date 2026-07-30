//! Cursor for traversing UTF-8 text.

#[derive(Debug)]
pub struct Cursor<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> Cursor<'a> {
    /// Creates a new cursor.
    pub fn new(text: &'a str) -> Self {
        Self { text, position: 0 }
    }

    /// Returns the current character.
    pub fn current(&self) -> Option<char> {
        self.text[self.position..].chars().next()
    }

    /// Returns the next character without advancing.
    pub fn peek(&self) -> Option<char> {
        let mut chars = self.text[self.position..].chars();

        chars.next()?;

        chars.next()
    }

    /// Advances the cursor by one character.
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;

        self.position += ch.len_utf8();

        Some(ch)
    }

    /// Returns the current byte position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns true if we've reached the end.
    pub fn is_eof(&self) -> bool {
        self.position >= self.text.len()
    }

    /// Returns the remaining text.
    pub fn remaining(&self) -> &'a str {
        &self.text[self.position..]
    }

    /// Resets the cursor.
    pub fn reset(&mut self) {
        self.position = 0;
    }
}
