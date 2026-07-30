//! Rule-based entity extraction.

use std::collections::HashSet;

/// Extracts candidate entities from a query.
pub struct EntityExtractor;

impl EntityExtractor {
    /// Extract entities from a query string.
    pub fn extract(query: &str) -> Vec<String> {
        const STOP_WORDS: &[&str] = &[
            "a", "an", "and", "are", "as", "at", "by", "for", "from", "how", "in", "is", "it",
            "of", "on", "or", "the", "to", "what", "when", "where", "which", "who", "why", "with",
        ];

        let mut seen = HashSet::new();
        let mut entities = Vec::new();

        for token in query
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
        {
            let token = token.to_lowercase();

            if STOP_WORDS.contains(&token.as_str()) {
                continue;
            }

            if seen.insert(token.clone()) {
                entities.push(token);
            }
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_keywords() {
        let entities = EntityExtractor::extract("Arch Linux package manager");

        assert_eq!(entities, vec!["arch", "linux", "package", "manager",]);
    }

    #[test]
    fn removes_duplicates() {
        let entities = EntityExtractor::extract("rust rust rust ownership");

        assert_eq!(entities, vec!["rust", "ownership",]);
    }

    #[test]
    fn ignores_stop_words() {
        let entities = EntityExtractor::extract("What is the Rust ownership model?");

        assert_eq!(entities, vec!["rust", "ownership", "model",]);
    }

    #[test]
    fn empty_query() {
        let entities = EntityExtractor::extract("");

        assert!(entities.is_empty());
    }
}
