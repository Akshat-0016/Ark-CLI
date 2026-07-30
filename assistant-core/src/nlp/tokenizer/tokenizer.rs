//! Converts raw text into tokens.

use crate::nlp::pipeline::TokenizerStage;

use super::cursor::Cursor;
use super::error::TokenizerError;
use super::scanner::Scanner;
use super::token::{Token, TokenKind};

pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Self {
        Self
    }

    fn read_word(&self, cursor: &mut Cursor) -> Token {
        let start = cursor.position();
        let mut lexeme = String::new();

        while let Some(ch) = cursor.current() {
            if !Scanner::is_identifier(ch) {
                break;
            }

            lexeme.push(ch);
            cursor.advance();
        }

        let end = cursor.position();

        Token::new(lexeme, TokenKind::Word, start, end)
    }

    fn read_number(&self, cursor: &mut Cursor) -> Token {
        let start = cursor.position();

        let mut lexeme = String::new();

        let mut seen_decimal = false;

        while let Some(ch) = cursor.current() {
            if Scanner::is_digit(ch) {
                lexeme.push(ch);
                cursor.advance();
            } else if ch == '.' && !seen_decimal {
                seen_decimal = true;
                lexeme.push(ch);
                cursor.advance();
            } else {
                break;
            }
        }

        let end = cursor.position();

        Token::new(lexeme, TokenKind::Number, start, end)
    }

    fn read_symbol(&self, cursor: &mut Cursor) -> Token {
        let start = cursor.position();

        let ch = cursor.advance().unwrap();

        let end = cursor.position();

        Token::new(ch.to_string(), TokenKind::Symbol, start, end)
    }

    fn read_unknown(&self, cursor: &mut Cursor) -> Token {
        let start = cursor.position();

        let ch = cursor.advance().unwrap();

        let end = cursor.position();

        Token::new(ch.to_string(), TokenKind::Unknown, start, end)
    }
}

impl TokenizerStage for Tokenizer {
    fn tokenize(&self, text: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut cursor = Cursor::new(text);

        let mut tokens = Vec::new();

        while !cursor.is_eof() {
            let ch = match cursor.current() {
                Some(ch) => ch,
                None => break,
            };

            if Scanner::is_whitespace(ch) {
                if Scanner::is_newline(ch) {
                    let start = cursor.position();

                    cursor.advance();

                    let end = cursor.position();

                    tokens.push(Token::new("\n".to_string(), TokenKind::Newline, start, end));
                } else {
                    cursor.advance();
                }

                continue;
            }

            if Scanner::is_identifier_start(ch) {
                tokens.push(self.read_word(&mut cursor));
                continue;
            }

            if Scanner::is_digit(ch) {
                tokens.push(self.read_number(&mut cursor));
                continue;
            }

            if Scanner::is_punctuation(ch) {
                tokens.push(self.read_symbol(&mut cursor));
                continue;
            }

            tokens.push(self.read_unknown(&mut cursor));
        }

        tokens.push(Token::new(
            String::new(),
            TokenKind::EndOfFile,
            cursor.position(),
            cursor.position(),
        ));

        Ok(tokens)
    }
}
