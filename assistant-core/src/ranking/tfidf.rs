use crate::{
    index::inverted_index::InvertedIndex,
    search::{document_match::DocumentMatch, result::SearchResult},
};

use super::ranker::Ranker;

pub struct TFIDFRanker;

impl TFIDFRanker {
    pub fn new() -> Self {
        Self
    }
}

impl Ranker for TFIDFRanker {
    fn rank(&self, index: &InvertedIndex, documents: &[DocumentMatch]) -> Vec<SearchResult> {
        let mut results = Vec::new();

        let total_documents = index.total_documents() as f32;

        for document in documents {
            let mut score = 0.0;

            for matcher in document.matches() {
                let tf = matcher.frequency() as f32;

                let df = index.document_frequency(matcher.term()) as f32;

                if df == 0.0 {
                    continue;
                }

                let idf = (total_documents / df).ln();

                score += tf * idf;
            }

            results.push(SearchResult::new(document.document_id(), score));
        }

        results.sort_by(|a, b| b.score().total_cmp(&a.score()));

        results
    }
}
