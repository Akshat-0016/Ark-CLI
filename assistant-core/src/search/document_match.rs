use super::matcher::Matcher;

#[derive(Debug, Clone)]
pub struct DocumentMatch {
    document_id: usize,
    matches: Vec<Matcher>,
}

impl DocumentMatch {
    pub fn new(document_id: usize) -> Self {
        Self {
            document_id,
            matches: Vec::new(),
        }
    }

    pub fn push(&mut self, matcher: Matcher) {
        self.matches.push(matcher);
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn matches(&self) -> &[Matcher] {
        &self.matches
    }

    pub fn total_frequency(&self) -> usize {
        self.matches.iter().map(|m| m.frequency()).sum()
    }
}
