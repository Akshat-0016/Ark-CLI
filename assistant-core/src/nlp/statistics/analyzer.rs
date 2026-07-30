use std::collections::{HashMap, HashSet};

use crate::nlp::tokenizer::token::{Token, TokenKind};

use super::report::StatisticsReport;

pub struct Analyzer;

impl Analyzer {
    pub fn analyze(tokens: &[Token]) -> StatisticsReport {
        let mut frequencies = HashMap::new();

        let mut vocabulary = HashSet::new();

        let mut words = 0;
        let mut numbers = 0;
        let mut symbols = 0;

        for token in tokens {
            match token.kind() {
                TokenKind::Word => {
                    words += 1;

                    vocabulary.insert(token.lexeme().to_owned());

                    *frequencies.entry(token.lexeme().to_owned()).or_insert(0) += 1;
                }

                TokenKind::Number => {
                    numbers += 1;
                }

                TokenKind::Symbol => {
                    symbols += 1;
                }

                _ => {}
            }
        }

        StatisticsReport {
            total_tokens: tokens.len(),
            total_words: words,
            total_numbers: numbers,
            total_symbols: symbols,
            vocabulary_size: vocabulary.len(),
            frequencies,
        }
    }
}
