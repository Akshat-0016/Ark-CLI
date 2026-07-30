use crate::nlp::{
    pipeline::PipelineStage,
    tokenizer::token::{Token, TokenKind},
};

use super::rules;

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }
}

impl PipelineStage for Normalizer {
    fn process(&self, mut tokens: Vec<Token>) -> Vec<Token> {
        for token in tokens.iter_mut() {
            match token.kind() {
                TokenKind::Word => {
                    rules::lowercase(token);
                }

                _ => {}
            }
        }

        tokens
    }
}
