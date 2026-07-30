#[derive(Debug, Clone)]
pub struct Matcher {
    document_id: usize,
    term: String,
    frequency: usize,
    positions: Vec<u32>,
}

impl Matcher {
    pub fn new(
        document_id: usize,
        term: impl Into<String>,
        frequency: usize,
        positions: Vec<u32>,
    ) -> Self {
        Self {
            document_id,
            term: term.into(),
            frequency,
            positions,
        }
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn term(&self) -> &str {
        &self.term
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn positions(&self) -> &[u32] {
        &self.positions
    }
}
