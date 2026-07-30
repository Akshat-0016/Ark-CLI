use crate::nlp::{
    normalizer::normalizer::Normalizer,
    pipeline::{PipelineStage, TokenizerStage},
    statistics::statistics::Statistics,
    tokenizer::tokenizer::Tokenizer,
};

#[test]
fn statistics_report() {
    let tokenizer = Tokenizer::new();
    let normalizer = Normalizer::new();
    let statistics = Statistics::new();

    let tokens = tokenizer.tokenize("Hello hello Rust ESP32 Rust").unwrap();

    let tokens = normalizer.process(tokens);

    let report = statistics.analyze(&tokens);

    assert_eq!(report.total_words, 5);
    assert_eq!(report.vocabulary_size, 3);
    assert_eq!(report.frequencies["rust"], 2);
}
