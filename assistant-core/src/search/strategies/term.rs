use crate::{
    index::inverted_index::InvertedIndex,
    search::{matcher::Matcher, query::Query},
};

pub struct TermStrategy;

impl TermStrategy {
    pub fn search(index: &InvertedIndex, query: &Query) -> Vec<Matcher> {
        let mut matches = Vec::new();

        for term in query.terms() {
            if let Some(postings) = index.get(term) {
                for posting in postings {
                    matches.push(Matcher::new(
                        posting.document_id(),
                        term.clone(),
                        posting.frequency(),
                        posting.positions().to_vec(),
                    ));
                }
            }
        }

        matches
    }
}
