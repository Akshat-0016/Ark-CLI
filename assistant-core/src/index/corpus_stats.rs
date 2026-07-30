#[derive(Debug, Clone, Default)]
pub struct CorpusStats {
    document_lengths: Vec<usize>,
    total_tokens: usize,
}

impl CorpusStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_document(&mut self, token_count: usize) {
        self.document_lengths.push(token_count);
        self.total_tokens += token_count;
    }

    pub fn document_length(&self, document_id: usize) -> usize {
        self.document_lengths[document_id]
    }

    pub fn total_documents(&self) -> usize {
        self.document_lengths.len()
    }

    pub fn average_document_length(&self) -> f32 {
        if self.document_lengths.is_empty() {
            return 0.0;
        }

        self.total_tokens as f32 / self.document_lengths.len() as f32
    }
}
