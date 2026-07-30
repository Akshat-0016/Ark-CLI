//! Query intent classification.

/// High-level user intent detected from a query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryIntent {
    /// General keyword or semantic search.
    Search,

    /// "What is Rust?"
    Definition,

    /// "Compare Rust and Zig."
    Comparison,

    /// "How is A related to B?"
    Relationship,

    /// "Open linux.md"
    Navigation,

    /// "Summarize this note."
    Summary,

    /// "List notes about AI."
    List,

    /// Unable to confidently determine intent.
    Unknown,
}

impl Default for QueryIntent {
    fn default() -> Self {
        Self::Search
    }
}

impl QueryIntent {
    /// Returns true if the intent requests retrieval of documents.
    pub fn requires_retrieval(self) -> bool {
        !matches!(self, Self::Unknown)
    }

    /// Returns true if the intent expects a synthesized response.
    pub fn requires_generation(self) -> bool {
        matches!(
            self,
            Self::Definition | Self::Comparison | Self::Relationship | Self::Summary
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_search() {
        assert_eq!(QueryIntent::default(), QueryIntent::Search);
    }

    #[test]
    fn retrieval_required() {
        assert!(QueryIntent::Search.requires_retrieval());
        assert!(QueryIntent::Summary.requires_retrieval());
        assert!(!QueryIntent::Unknown.requires_retrieval());
    }

    #[test]
    fn generation_required() {
        assert!(QueryIntent::Definition.requires_generation());
        assert!(QueryIntent::Comparison.requires_generation());
        assert!(QueryIntent::Relationship.requires_generation());
        assert!(QueryIntent::Summary.requires_generation());

        assert!(!QueryIntent::Search.requires_generation());
        assert!(!QueryIntent::Navigation.requires_generation());
        assert!(!QueryIntent::List.requires_generation());
    }
}
