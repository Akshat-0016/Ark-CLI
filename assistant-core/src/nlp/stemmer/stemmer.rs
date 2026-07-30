use crate::nlp::{pipeline::PipelineStage, tokenizer::token::Token};

pub struct Stemmer;

impl Stemmer {
    pub fn new() -> Self {
        Self
    }

    pub fn stem(&self, word: &str) -> String {
        if let Some(stripped) = word.strip_suffix("ing") {
            stripped.to_owned()
        } else if let Some(stripped) = word.strip_suffix("ed") {
            stripped.to_owned()
        } else if let Some(stripped) = word.strip_suffix("es") {
            stripped.to_owned()
        } else if word.ends_with('s') && word.len() > 3 {
            word[..word.len() - 1].to_owned()
        } else {
            word.to_owned()
        }
    }
}

impl PipelineStage for Stemmer {
    fn process(&self, mut tokens: Vec<Token>) -> Vec<Token> {
        for token in &mut tokens {
            *token.lexeme_mut() = self.stem(token.lexeme());
        }

        tokens
    }
}
