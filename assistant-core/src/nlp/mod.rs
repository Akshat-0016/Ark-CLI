pub mod normalizer;
pub mod pipeline;
pub mod sentence;
pub mod statistics;
pub mod stemmer;
pub mod stopwords;
pub mod tokenizer;

use crate::nlp::stemmer::stemmer::Stemmer;
use crate::nlp::stopwords::filter::StopwordFilter;
use crate::nlp::{
    pipeline::Pipeline,
    tokenizer::{
        error::TokenizerError,
        token::{Token, TokenKind},
        tokenizer::Tokenizer,
    },
};

pub fn process(text: &str) -> Result<Vec<Token>, TokenizerError> {
    let mut pipeline = Pipeline::new(Tokenizer::new());

    pipeline.add_stage(StopwordFilter::new());
    pipeline.add_stage(Stemmer::new());

    pipeline.run(text)
}
pub fn process_text(text: &str) -> Result<String, TokenizerError> {
    let tokens = process(text)?;

    Ok(tokens
        .into_iter()
        .filter(|t| t.kind() != TokenKind::EndOfFile)
        .map(|t| t.lexeme().to_owned())
        .collect::<Vec<_>>()
        .join(" "))
}
pub fn process_words(text: &str) -> Result<Vec<String>, TokenizerError> {
    Ok(process(text)?
        .into_iter()
        .filter(|t| t.kind() == TokenKind::Word)
        .map(|t| t.lexeme().to_owned())
        .collect())
}
