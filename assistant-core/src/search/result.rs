#[derive(Debug, Clone)]
pub struct SearchResult {
    document_id: usize,
    score: f32,
}

impl SearchResult {
    pub fn new(document_id: usize, score: f32) -> Self {
        Self { document_id, score }
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}
