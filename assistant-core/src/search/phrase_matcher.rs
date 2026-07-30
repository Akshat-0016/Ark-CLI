use std::collections::HashMap;

use super::matcher::Matcher;

/// Performs positional verification for phrase searches.
///
/// The searcher is responsible for retrieving candidate postings.
/// This matcher verifies that the positions represent an exact phrase.
pub struct PhraseMatcher;

impl PhraseMatcher {
    pub fn filter(terms: &[String], matches: Vec<Matcher>) -> Vec<Matcher> {
        let mut grouped: HashMap<usize, Vec<Matcher>> = HashMap::new();

        for matcher in matches {
            grouped
                .entry(matcher.document_id())
                .or_default()
                .push(matcher);
        }

        let mut verified = Vec::new();

        for (_, document_matches) in grouped {
            if Self::is_phrase_match(terms, &document_matches) {
                verified.extend(document_matches);
            }
        }

        verified
    }

    fn is_phrase_match(terms: &[String], matches: &[Matcher]) -> bool {
        if terms.is_empty() {
            return false;
        }

        let mut postings: HashMap<&str, &[u32]> = HashMap::new();

        for matcher in matches {
            postings.insert(matcher.term(), matcher.positions());
        }

        let Some(first_positions) = postings.get(terms[0].as_str()) else {
            return false;
        };

        for &start in *first_positions {
            let mut valid = true;

            for (offset, term) in terms.iter().enumerate().skip(1) {
                let expected = start + offset as u32;

                match postings.get(term.as_str()) {
                    Some(positions) if positions.contains(&expected) => {}

                    _ => {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                return true;
            }
        }

        false
    }
}
