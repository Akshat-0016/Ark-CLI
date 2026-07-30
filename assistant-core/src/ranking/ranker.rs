use crate::{
    index::inverted_index::InvertedIndex,
    search::{document_match::DocumentMatch, result::SearchResult},
};

pub trait Ranker {
    fn rank(&self, index: &InvertedIndex, documents: &[DocumentMatch]) -> Vec<SearchResult>;
}
