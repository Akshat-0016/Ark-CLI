use std::collections::HashMap;

use super::{document_match::DocumentMatch, matcher::Matcher};

pub struct Aggregator;

impl Aggregator {
    pub fn aggregate(matches: Vec<Matcher>) -> Vec<DocumentMatch> {
        let mut grouped: HashMap<usize, DocumentMatch> = HashMap::new();

        for matcher in matches {
            grouped
                .entry(matcher.document_id())
                .or_insert_with(|| DocumentMatch::new(matcher.document_id()))
                .push(matcher);
        }

        let mut docs: Vec<DocumentMatch> = grouped.into_values().collect();

        docs.sort_by_key(|d| d.document_id());

        docs
    }
}
