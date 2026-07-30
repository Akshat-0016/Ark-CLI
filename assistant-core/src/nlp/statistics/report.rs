use std::collections::HashMap;

#[derive(Debug)]
pub struct StatisticsReport {
    pub total_tokens: usize,
    pub total_words: usize,
    pub total_numbers: usize,
    pub total_symbols: usize,
    pub vocabulary_size: usize,
    pub frequencies: HashMap<String, usize>,
}
