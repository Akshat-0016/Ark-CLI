use std::collections::HashMap;

use super::{corpus_stats::CorpusStats, posting::Posting};

#[derive(Debug)]
pub struct InvertedIndex {
    index: HashMap<String, Vec<Posting>>,
    stats: CorpusStats,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            stats: CorpusStats::new(),
        }
    }

    pub fn insert(&mut self, term: &str, document_id: usize, position: u32) {
        let postings = self.index.entry(term.to_owned()).or_default();

        if let Some(posting) = postings.iter_mut().find(|p| p.document_id() == document_id) {
            posting.increment(position);
        } else {
            postings.push(Posting::new(document_id, position));
        }
    }

    pub fn get(&self, term: &str) -> Option<&Vec<Posting>> {
        self.index.get(term)
    }

    pub fn terms(&self) -> usize {
        self.index.len()
    }

    pub fn contains(&self, term: &str) -> bool {
        self.index.contains_key(term)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<Posting>)> {
        self.index.iter()
    }

    pub fn postings(&self, term: &str) -> Option<&[Posting]> {
        self.index.get(term).map(Vec::as_slice)
    }
    pub fn positions(&self, term: &str, document_id: usize) -> Option<&[u32]> {
        self.index
            .get(term)?
            .iter()
            .find(|p| p.document_id() == document_id)
            .map(|p| p.positions())
    }

    pub fn add_document(&mut self, token_count: usize) {
        self.stats.add_document(token_count);
    }

    pub fn document_length(&self, document_id: usize) -> usize {
        self.stats.document_length(document_id)
    }

    pub fn total_documents(&self) -> usize {
        self.stats.total_documents()
    }

    pub fn average_document_length(&self) -> f32 {
        self.stats.average_document_length()
    }

    pub fn document_frequency(&self, term: &str) -> usize {
        self.postings(term).map(|p| p.len()).unwrap_or(0)
    }
}
