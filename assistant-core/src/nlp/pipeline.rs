//! Common traits for NLP processing stages.

use crate::nlp::tokenizer::{error::TokenizerError, token::Token};

pub trait TokenizerStage {
    fn tokenize(&self, text: &str) -> Result<Vec<Token>, TokenizerError>;
}

pub trait PipelineStage {
    fn process(&self, tokens: Vec<Token>) -> Vec<Token>;
}

pub struct Pipeline<T> {
    tokenizer: T,
    stages: Vec<Box<dyn PipelineStage>>,
}

impl<T: TokenizerStage> Pipeline<T> {
    pub fn new(tokenizer: T) -> Self {
        Self {
            tokenizer,
            stages: Vec::new(),
        }
    }

    pub fn add_stage<S: PipelineStage + 'static>(&mut self, stage: S) {
        self.stages.push(Box::new(stage));
    }

    pub fn run(&self, text: &str) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = self.tokenizer.tokenize(text)?;

        for stage in &self.stages {
            tokens = stage.process(tokens);
        }

        Ok(tokens)
    }
}
