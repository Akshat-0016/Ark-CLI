use crate::index::document::Document;

/// A document prepared for inclusion in an AI context.
#[derive(Debug, Clone)]
pub struct ContextDocument {
    document_id: usize,
    title: String,
    text: String,
    estimated_tokens: usize,
}

impl ContextDocument {
    pub fn new(document_id: usize, document: &Document) -> Self {
        Self {
            document_id,
            title: document.title().to_owned(),
            text: document.text().to_owned(),
            estimated_tokens: estimate_tokens(document.text()),
        }
    }

    pub fn document_id(&self) -> usize {
        self.document_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn estimated_tokens(&self) -> usize {
        self.estimated_tokens
    }
}

fn estimate_tokens(text: &str) -> usize {
    ((text.len() as f32) / 4.0).ceil() as usize
}

/// Final context supplied to the reasoning engine.
#[derive(Debug, Default)]
pub struct Context {
    primary: Vec<ContextDocument>,
    related: Vec<ContextDocument>,
    total_tokens: usize,
}

impl Context {
    pub fn primary(&self) -> &[ContextDocument] {
        &self.primary
    }

    pub fn related(&self) -> &[ContextDocument] {
        &self.related
    }

    pub fn documents(&self) -> impl Iterator<Item = &ContextDocument> {
        self.primary.iter().chain(self.related.iter())
    }

    pub fn total_tokens(&self) -> usize {
        self.total_tokens
    }

    pub fn try_add_primary(&mut self, document: ContextDocument, token_budget: usize) -> bool {
        if self.total_tokens + document.estimated_tokens() > token_budget {
            return false;
        }

        self.total_tokens += document.estimated_tokens();
        self.primary.push(document);
        true
    }

    pub fn try_add_related(&mut self, document: ContextDocument, token_budget: usize) -> bool {
        if self.total_tokens + document.estimated_tokens() > token_budget {
            return false;
        }

        self.total_tokens += document.estimated_tokens();
        self.related.push(document);
        true
    }
}
