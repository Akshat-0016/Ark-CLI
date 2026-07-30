#[derive(Debug, Clone)]
pub struct Posting {
    pub document_id: usize,
    pub frequency: usize,
    pub positions: Vec<u32>,
}

impl Posting {
    pub fn new(document_id: usize, position: u32) -> Self {
        Self {
            document_id,
            frequency: 1,
            positions: vec![position],
        }
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn frequency(&self) -> usize {
        self.frequency
    }

    pub fn positions(&self) -> &[u32] {
        &self.positions
    }

    pub fn increment(&mut self, position: u32) {
        self.frequency += 1;
        self.positions.push(position);
    }
}
