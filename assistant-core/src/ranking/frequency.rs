use crate::{
    index::inverted_index::InvertedIndex,
    search::{document_match::DocumentMatch, result::SearchResult},
};

use super::ranker::Ranker;

pub struct FrequencyRanker;

impl FrequencyRanker {
    pub fn new() -> Self {
        Self
    }
}

impl Ranker for FrequencyRanker {
    fn rank(&self, _: &InvertedIndex, documents: &[DocumentMatch]) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for document in documents {
            results.push(SearchResult::new(
                document.document_id(),
                document.total_frequency() as f32,
            ));
        }

        results.sort_by(|a, b| b.score().total_cmp(&a.score()));

        results
    }
}
