//! Result of query analysis.

use super::intent::QueryIntent;

/// Structured representation of a user query.
#[derive(Debug, Clone)]
pub struct AnalyzedQuery {
    original: String,
    normalized: String,
    intent: QueryIntent,
    entities: Vec<String>,
    expanded_terms: Vec<String>,
}

impl AnalyzedQuery {
    /// Creates a new analyzed query.
    pub fn new(
        original: impl Into<String>,
        normalized: impl Into<String>,
        intent: QueryIntent,
        entities: Vec<String>,
        expanded_terms: Vec<String>,
    ) -> Self {
        Self {
            original: original.into(),
            normalized: normalized.into(),
            intent,
            entities,
            expanded_terms,
        }
    }

    /// Original user input.
    pub fn original(&self) -> &str {
        &self.original
    }

    /// Normalized query text.
    pub fn normalized(&self) -> &str {
        &self.normalized
    }

    /// Detected intent.
    pub fn intent(&self) -> QueryIntent {
        self.intent
    }

    /// Extracted entities.
    pub fn entities(&self) -> &[String] {
        &self.entities
    }

    /// Expanded search terms.
    pub fn expanded_terms(&self) -> &[String] {
        &self.expanded_terms
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::QueryIntent;

    #[test]
    fn analyzed_query_fields() {
        let query = AnalyzedQuery::new(
            "Arch Linux",
            "arch linux",
            QueryIntent::Search,
            vec!["arch".into(), "linux".into()],
            vec!["arch".into(), "linux".into(), "arch linux".into()],
        );

        assert_eq!(query.original(), "Arch Linux");
        assert_eq!(query.normalized(), "arch linux");
        assert_eq!(query.intent(), QueryIntent::Search);

        assert_eq!(query.entities().len(), 2);
        assert_eq!(query.expanded_terms().len(), 3);
    }
}
