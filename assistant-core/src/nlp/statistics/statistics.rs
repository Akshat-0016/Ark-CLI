use crate::nlp::tokenizer::token::Token;

use super::{analyzer::Analyzer, report::StatisticsReport};

pub struct Statistics;

impl Statistics {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, tokens: &[Token]) -> StatisticsReport {
        Analyzer::analyze(tokens)
    }
}
