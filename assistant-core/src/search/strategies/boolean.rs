use std::collections::HashMap;

use crate::{
    index::inverted_index::InvertedIndex,
    search::{
        matcher::Matcher,
        query::{Query, QueryOperator},
    },
};

pub struct BooleanStrategy;

impl BooleanStrategy {
    pub fn search(index: &InvertedIndex, query: &Query) -> Vec<Matcher> {
        let mut groups: HashMap<usize, Vec<Matcher>> = HashMap::new();

        for term in query.terms() {
            if let Some(postings) = index.get(term) {
                for posting in postings {
                    groups
                        .entry(posting.document_id())
                        .or_default()
                        .push(Matcher::new(
                            posting.document_id(),
                            term.clone(),
                            posting.frequency(),
                            posting.positions().to_vec(),
                        ));
                }
            }
        }

        match query.operator() {
            QueryOperator::Or => groups.into_values().flatten().collect(),

            QueryOperator::And => groups
                .into_values()
                .filter(|m| m.len() == query.terms().len())
                .flatten()
                .collect(),

            QueryOperator::Not => {
                todo!("NOT strategy will be implemented later")
            }
        }
    }
}
