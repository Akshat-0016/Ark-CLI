use std::collections::HashSet;

use crate::index::inverted_index::InvertedIndex;

use super::matcher::Matcher;

/// Low-level retrieval layer.
///
/// The Searcher knows **only** how to retrieve postings from the index.
/// It does not know anything about:
///
/// - Query parsing
/// - Boolean logic
/// - Ranking
/// - AST evaluation
///
/// Those responsibilities belong to higher layers.
pub struct Searcher;

impl Searcher {
    pub fn new() -> Self {
        Self
    }

    /// Retrieve every occurrence of a single term.
    pub fn term(&self, index: &InvertedIndex, term: &str) -> Vec<Matcher> {
        let mut matches = Vec::new();

        if let Some(postings) = index.postings(term) {
            for posting in postings {
                matches.push(Matcher::new(
                    posting.document_id(),
                    term,
                    posting.frequency(),
                    posting.positions().to_vec(),
                ));
            }
        }

        matches
    }

    /// Retrieve documents containing an exact phrase.
    ///
    /// This implementation performs positional matching using the
    /// posting lists already stored in the inverted index.
    pub fn phrase(&self, index: &InvertedIndex, terms: &[String]) -> Vec<Matcher> {
        if terms.is_empty() {
            return Vec::new();
        }

        if terms.len() == 1 {
            return self.term(index, &terms[0]);
        }

        // ----------------------------------------------------------
        // Find documents containing every term
        // ----------------------------------------------------------

        let mut candidate_docs: Option<HashSet<usize>> = None;

        for term in terms {
            let docs: HashSet<_> = match index.postings(term) {
                Some(postings) => postings.iter().map(|p| p.document_id()).collect(),

                None => return Vec::new(),
            };

            candidate_docs = Some(match candidate_docs {
                None => docs,

                Some(existing) => existing.intersection(&docs).copied().collect(),
            });
        }

        let Some(candidate_docs) = candidate_docs else {
            return Vec::new();
        };

        // ----------------------------------------------------------
        // Positional verification
        // ----------------------------------------------------------

        let mut results = Vec::new();

        for document_id in candidate_docs {
            let mut position_maps: Vec<&[u32]> = Vec::new();

            let mut valid = true;

            for term in terms {
                match index.positions(term, document_id) {
                    Some(positions) => position_maps.push(positions),

                    None => {
                        valid = false;
                        break;
                    }
                }
            }

            if !valid {
                continue;
            }

            let mut matched_positions = Vec::new();

            'outer: for &start in position_maps[0] {
                for (offset, positions) in position_maps.iter().enumerate().skip(1) {
                    let expected = start + offset as u32;

                    if !positions.contains(&expected) {
                        continue 'outer;
                    }
                }

                matched_positions.push(start);
            }

            if !matched_positions.is_empty() {
                results.push(Matcher::new(
                    document_id,
                    terms.join(" "),
                    matched_positions.len(),
                    matched_positions,
                ));
            }
        }

        results.sort_by_key(|m| m.document_id());

        results
    }

    /// Direct access to postings.
    pub fn postings<'a>(
        &self,
        index: &'a InvertedIndex,
        term: &str,
    ) -> Option<&'a [crate::index::posting::Posting]> {
        index.postings(term)
    }

    /// Returns every document containing the term.
    pub fn documents(&self, index: &InvertedIndex, term: &str) -> Vec<usize> {
        index
            .postings(term)
            .map(|postings| postings.iter().map(|p| p.document_id()).collect())
            .unwrap_or_default()
    }

    /// Checks whether a term exists in the index.
    pub fn contains(&self, index: &InvertedIndex, term: &str) -> bool {
        index.contains(term)
    }

    /// Returns the document frequency of a term.
    pub fn document_frequency(&self, index: &InvertedIndex, term: &str) -> usize {
        index.document_frequency(term)
    }
}
