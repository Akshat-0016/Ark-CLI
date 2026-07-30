use crate::nlp::{pipeline::PipelineStage, tokenizer::token::Token};

use super::list::STOPWORDS;

pub struct StopwordFilter;

impl StopwordFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn is_stopword(&self, word: &str) -> bool {
        STOPWORDS.contains(&word)
    }

    pub fn filter(&self, tokens: Vec<Token>) -> Vec<Token> {
        tokens
            .into_iter()
            .filter(|token| !self.is_stopword(token.lexeme()))
            .collect()
    }
}

impl PipelineStage for StopwordFilter {
    fn process(&self, tokens: Vec<Token>) -> Vec<Token> {
        self.filter(tokens)
    }
}
