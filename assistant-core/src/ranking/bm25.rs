use std::collections::HashMap;

use crate::{
    index::inverted_index::InvertedIndex,
    ranking::ranker::Ranker,
    search::{document_match::DocumentMatch, result::SearchResult},
};

/// Okapi BM25 ranker.
///
/// References:
/// - Robertson & Walker (1994)
///
/// Default parameters:
/// - k1 = 1.5
/// - b = 0.75
pub struct BM25Ranker {
    k1: f32,
    b: f32,
}

impl BM25Ranker {
    pub fn new() -> Self {
        Self { k1: 1.5, b: 0.75 }
    }

    pub fn with_parameters(k1: f32, b: f32) -> Self {
        Self { k1, b }
    }

    fn average_document_length(&self, index: &InvertedIndex) -> f32 {
        let total_docs = index.total_documents();

        if total_docs == 0 {
            return 0.0;
        }

        let mut total_terms = 0usize;

        for document_id in 0..total_docs {
            total_terms += index.document_length(document_id);
        }

        total_terms as f32 / total_docs as f32
    }
}

impl Default for BM25Ranker {
    fn default() -> Self {
        Self::new()
    }
}

impl Ranker for BM25Ranker {
    fn rank(&self, index: &InvertedIndex, documents: &[DocumentMatch]) -> Vec<SearchResult> {
        let avgdl = self.average_document_length(index);

        let total_documents = index.total_documents() as f32;

        let mut results = Vec::new();

        for document in documents {
            let document_length = index.document_length(document.document_id()) as f32;

            let mut score = 0.0;

            let mut frequencies: HashMap<&str, usize> = HashMap::new();

            for matcher in document.matches() {
                *frequencies.entry(matcher.term()).or_insert(0) += matcher.frequency();
            }

            for (term, tf) in frequencies {
                let df = index.document_frequency(term) as f32;

                if df == 0.0 {
                    continue;
                }

                let idf = ((total_documents - df + 0.5) / (df + 0.5) + 1.0).ln();

                let tf = tf as f32;

                let numerator = tf * (self.k1 + 1.0);

                let denominator =
                    tf + self.k1 * (1.0 - self.b + self.b * (document_length / avgdl.max(1.0)));

                score += idf * (numerator / denominator);
            }

            results.push(SearchResult::new(document.document_id(), score));
        }

        results.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());

        results
    }
}
