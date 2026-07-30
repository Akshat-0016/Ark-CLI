use crate::{
    index::inverted_index::InvertedIndex,
    search::{
        matcher::Matcher,
        phrase_matcher::PhraseMatcher,
        query::{Query, QueryOperator},
    },
};

use super::boolean::BooleanStrategy;

pub struct PhraseStrategy;

impl PhraseStrategy {
    pub fn search(index: &InvertedIndex, query: &Query) -> Vec<Matcher> {
        let query = query.with_operator(QueryOperator::And);

        let matches = BooleanStrategy::search(index, &query);

        PhraseMatcher::filter(&query, matches)
    }
}
