use crate::index::{document::Document, inverted_index::InvertedIndex};

#[derive(Debug)]
pub struct BuildArtifacts {
    pub index: InvertedIndex,
    pub documents: Vec<Document>,
}

impl BuildArtifacts {
    pub fn new(index: InvertedIndex, documents: Vec<Document>) -> Self {
        Self { index, documents }
    }
}
