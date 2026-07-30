//! Query expansion.

use std::collections::HashSet;

/// Expands extracted entities into a richer search vocabulary.
pub struct QueryExpansion;

impl QueryExpansion {
    /// Expand a list of extracted entities.
    pub fn expand(entities: &[String]) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut expanded = Vec::new();

        for entity in entities {
            Self::push(entity, &mut expanded, &mut seen);

            // -----------------------------------------------------------------
            // Future expansion sources:
            //
            // - Synonyms
            // - Knowledge graph neighbors
            // - Aliases
            // - Tags
            // - Acronyms
            // - Embedding neighbors
            // -----------------------------------------------------------------
        }

        expanded
    }

    fn push(term: &str, expanded: &mut Vec<String>, seen: &mut HashSet<String>) {
        if seen.insert(term.to_owned()) {
            expanded.push(term.to_owned());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_single_term() {
        let expanded = QueryExpansion::expand(&["rust".to_string()]);

        assert_eq!(expanded, vec!["rust".to_string(),]);
    }

    #[test]
    fn removes_duplicates() {
        let expanded = QueryExpansion::expand(&["rust".into(), "rust".into(), "linux".into()]);

        assert_eq!(expanded, vec!["rust".to_string(), "linux".to_string(),]);
    }

    #[test]
    fn empty_entities() {
        let expanded = QueryExpansion::expand(&[]);

        assert!(expanded.is_empty());
    }
}
