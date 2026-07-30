#[derive(Debug, Clone)]
pub struct Score {
    document_id: usize,
    value: f32,
}

impl Score {
    pub fn new(document_id: usize, value: f32) -> Self {
        Self { document_id, value }
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}
