use crate::index::document::Document;

#[derive(Debug, Clone, Copy)]
pub struct RankedDocument<'a> {
    document: &'a Document,
    score: f32,
}

impl<'a> RankedDocument<'a> {
    pub fn new(document: &'a Document, score: f32) -> Self {
        Self { document, score }
    }

    pub fn document(&self) -> &'a Document {
        self.document
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}
