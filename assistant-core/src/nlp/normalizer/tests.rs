use crate::nlp::{
    normalizer::normalizer::Normalizer,
    pipeline::{PipelineStage, TokenizerStage},
    tokenizer::tokenizer::Tokenizer,
};

#[test]
fn normalize_words() {
    let tokenizer = Tokenizer::new();
    let normalizer = Normalizer::new();

    let tokens = tokenizer.tokenize("Hello RUST Esp32").unwrap();

    let tokens = normalizer.process(tokens);

    assert_eq!(tokens[0].lexeme(), "hello");
    assert_eq!(tokens[1].lexeme(), "rust");
    assert_eq!(tokens[2].lexeme(), "esp32");
}
