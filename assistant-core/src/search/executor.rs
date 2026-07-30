use std::collections::{HashMap, HashSet};

use crate::{
    index::inverted_index::InvertedIndex,
    search::{
        document_match::DocumentMatch, matcher::Matcher, plan::ExecutionPlan, searcher::Searcher,
    },
};

/// Executes an ExecutionPlan.
///
/// The Executor knows how to execute plans.
/// It does NOT know:
///
/// - how to tokenize
/// - how to parse
/// - how to rank
///
/// It is the bridge between the Planner and the Searcher.
pub struct Executor<'a> {
    searcher: &'a Searcher,
    index: &'a InvertedIndex,
}

impl<'a> Executor<'a> {
    pub fn new(searcher: &'a Searcher, index: &'a InvertedIndex) -> Self {
        Self { searcher, index }
    }

    /// Execute a plan.
    pub fn execute(&self, plan: &ExecutionPlan) -> Vec<DocumentMatch> {
        match plan {
            ExecutionPlan::Term(term) => self.aggregate(self.searcher.term(self.index, term)),

            ExecutionPlan::Phrase(terms) => self.aggregate(self.searcher.phrase(self.index, terms)),

            ExecutionPlan::Intersect(left, right) => {
                self.intersection(self.execute(left), self.execute(right))
            }

            ExecutionPlan::Union(left, right) => {
                self.union(self.execute(left), self.execute(right))
            }

            ExecutionPlan::Difference(node) => self.difference(self.execute(node)),
        }
    }

    fn aggregate(&self, matches: Vec<Matcher>) -> Vec<DocumentMatch> {
        let mut grouped: HashMap<usize, DocumentMatch> = HashMap::new();

        for matcher in matches {
            grouped
                .entry(matcher.document_id())
                .or_insert_with(|| DocumentMatch::new(matcher.document_id()))
                .push(matcher);
        }

        let mut docs: Vec<_> = grouped.into_values().collect();

        docs.sort_by_key(|d| d.document_id());

        docs
    }

    fn intersection(
        &self,
        left: Vec<DocumentMatch>,
        right: Vec<DocumentMatch>,
    ) -> Vec<DocumentMatch> {
        let mut right_map = HashMap::new();

        for doc in right {
            right_map.insert(doc.document_id(), doc);
        }

        let mut result = Vec::new();

        for mut doc in left {
            if let Some(other) = right_map.remove(&doc.document_id()) {
                for matcher in other.matches() {
                    doc.push(matcher.clone());
                }

                result.push(doc);
            }
        }

        result
    }

    fn union(&self, left: Vec<DocumentMatch>, right: Vec<DocumentMatch>) -> Vec<DocumentMatch> {
        let mut docs = HashMap::new();

        for doc in left {
            docs.insert(doc.document_id(), doc);
        }

        for doc in right {
            docs.entry(doc.document_id())
                .and_modify(|existing| {
                    for matcher in doc.matches() {
                        existing.push(matcher.clone());
                    }
                })
                .or_insert(doc);
        }

        let mut result: Vec<_> = docs.into_values().collect();

        result.sort_by_key(|d| d.document_id());

        result
    }

    fn difference(&self, excluded: Vec<DocumentMatch>) -> Vec<DocumentMatch> {
        let excluded: HashSet<_> = excluded.iter().map(|d| d.document_id()).collect();

        let mut result = Vec::new();

        for document_id in 0..self.index.total_documents() {
            if !excluded.contains(&document_id) {
                result.push(DocumentMatch::new(document_id));
            }
        }

        result
    }
}
