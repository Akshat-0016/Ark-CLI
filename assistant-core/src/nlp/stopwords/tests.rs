use crate::nlp::{
    pipeline::TokenizerStage,
    stopwords::filter::StopwordFilter,
    tokenizer::{token::TokenKind, tokenizer::Tokenizer},
};

#[test]
fn removes_stopwords() {
    let tokenizer = Tokenizer::new();
    let filter = StopwordFilter::new();

    let tokens = tokenizer.tokenize("Rust is the best language").unwrap();

    let filtered = filter.filter(tokens);

    let words: Vec<_> = filtered
        .iter()
        .filter(|t| t.kind() == TokenKind::Word)
        .map(|t| t.lexeme().to_string())
        .collect();

    assert_eq!(words, vec!["rust", "best", "language",]);
}
