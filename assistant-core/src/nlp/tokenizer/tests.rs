use super::token::TokenKind;
use super::tokenizer::Tokenizer;
use crate::nlp::pipeline::TokenizerStage;

#[test]
fn tokenize_words() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("Hello Rust").unwrap();

    assert_eq!(tokens[0].kind(), TokenKind::Word);
    assert_eq!(tokens[0].lexeme(), "Hello");

    assert_eq!(tokens[1].kind(), TokenKind::Word);
    assert_eq!(tokens[1].lexeme(), "Rust");
}

#[test]
fn tokenize_numbers() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("2026 3.14").unwrap();

    assert_eq!(tokens[0].kind(), TokenKind::Number);
    assert_eq!(tokens[0].lexeme(), "2026");

    assert_eq!(tokens[1].kind(), TokenKind::Number);
    assert_eq!(tokens[1].lexeme(), "3.14");
}

#[test]
fn tokenize_symbols() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("!@#").unwrap();

    assert_eq!(tokens[0].kind(), TokenKind::Symbol);
    assert_eq!(tokens[1].kind(), TokenKind::Symbol);
    assert_eq!(tokens[2].kind(), TokenKind::Symbol);
}

#[test]
fn tokenize_empty() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("").unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind(), TokenKind::EndOfFile);
}

#[test]
fn tokenize_newline() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("Hello\nWorld").unwrap();

    assert_eq!(tokens[1].kind(), TokenKind::Newline);
}

#[test]
fn tokenize_mixed_input() {
    let tokenizer = Tokenizer::new();

    let tokens = tokenizer.tokenize("Hello, ESP32 2026!").unwrap();

    assert_eq!(tokens[0].kind(), TokenKind::Word);
    assert_eq!(tokens[1].kind(), TokenKind::Symbol);
    assert_eq!(tokens[2].kind(), TokenKind::Word);
    assert_eq!(tokens[3].kind(), TokenKind::Number);
    assert_eq!(tokens[4].kind(), TokenKind::Symbol);
}
