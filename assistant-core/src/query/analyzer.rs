//! Query analyzer.

use super::{
    analyzed_query::AnalyzedQuery, entities::EntityExtractor, expansion::QueryExpansion,
    intent::QueryIntent,
};

/// Analyzes raw user queries into a structured representation.
pub struct QueryAnalyzer;

impl QueryAnalyzer {
    /// Analyze a query.
    pub fn analyze(query: &str) -> AnalyzedQuery {
        let normalized = Self::normalize(query);

        let intent = Self::detect_intent(&normalized);

        let entities = EntityExtractor::extract(&normalized);

        let expanded_terms = QueryExpansion::expand(&entities);

        AnalyzedQuery::new(query, normalized, intent, entities, expanded_terms)
    }

    /// Normalize the query.
    fn normalize(query: &str) -> String {
        query
            .trim()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Detect user intent.
    fn detect_intent(query: &str) -> QueryIntent {
        if query.starts_with("what is")
            || query.starts_with("who is")
            || query.starts_with("define")
        {
            QueryIntent::Definition
        } else if query.starts_with("compare") {
            QueryIntent::Comparison
        } else if query.starts_with("summarize") {
            QueryIntent::Summary
        } else if query.starts_with("list") || query.starts_with("show") {
            QueryIntent::List
        } else if query.starts_with("open") || query.starts_with("goto") {
            QueryIntent::Navigation
        } else if query.contains("relationship") || query.contains("related") {
            QueryIntent::Relationship
        } else {
            QueryIntent::Search
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_query() {
        let analyzed = QueryAnalyzer::analyze("   Arch    Linux   ");

        assert_eq!(analyzed.normalized(), "arch linux");
    }

    #[test]
    fn detects_definition() {
        let analyzed = QueryAnalyzer::analyze("What is Rust");

        assert_eq!(analyzed.intent(), QueryIntent::Definition);
    }

    #[test]
    fn detects_search() {
        let analyzed = QueryAnalyzer::analyze("arch linux");

        assert_eq!(analyzed.intent(), QueryIntent::Search);
    }

    #[test]
    fn extracts_entities() {
        let analyzed = QueryAnalyzer::analyze("Arch Linux package manager");

        assert!(analyzed.entities().contains(&"arch".to_string()));
        assert!(analyzed.entities().contains(&"linux".to_string()));
    }

    #[test]
    fn expands_query() {
        let analyzed = QueryAnalyzer::analyze("Rust ownership");

        assert!(!analyzed.expanded_terms().is_empty());
    }
}
