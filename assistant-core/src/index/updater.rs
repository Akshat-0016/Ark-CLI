use crate::{index::inverted_index::InvertedIndex, storage::document::Document};

/// Incrementally updates an existing inverted index.
///
/// Unlike the IndexBuilder, this component modifies an already
/// constructed index.
///
/// Future responsibilities:
///
/// - Add new documents
/// - Remove deleted documents
/// - Re-index modified documents
/// - Update statistics
/// - Maintain document lengths
pub struct IndexUpdater;

impl IndexUpdater {
    pub fn new() -> Self {
        Self
    }

    /// Insert a newly created document.
    pub fn insert(&self, index: &mut InvertedIndex, document: &Document) {
        // TODO:
        //
        // 1. Tokenize document
        // 2. Normalize tokens
        // 3. Insert postings
        // 4. Update document length
        // 5. Update vocabulary statistics

        todo!("incremental insert");
    }

    /// Remove a document from the index.
    pub fn remove(&self, index: &mut InvertedIndex, document_id: usize) {
        // TODO:
        //
        // Remove every posting belonging
        // to this document.

        let _ = (index, document_id);

        todo!("incremental remove");
    }

    /// Re-index a modified document.
    pub fn update(&self, index: &mut InvertedIndex, document: &Document) {
        self.remove(index, document.id());
        self.insert(index, document);
    }
}

impl Default for IndexUpdater {
    fn default() -> Self {
        Self::new()
    }
}
