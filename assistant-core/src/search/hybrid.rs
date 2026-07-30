use std::collections::HashMap;

use crate::{graph::traversal::Traversal, ranking::result::SearchResult};

/// Combines lexical and graph scores.
///
/// Final score:
///
/// final = α * lexical + β * semantic
///
/// where:
///
/// lexical  -> BM25
/// semantic -> graph/context score
pub struct HybridRanker {
    lexical_weight: f32,
    semantic_weight: f32,
}

impl HybridRanker {
    pub fn new() -> Self {
        Self {
            lexical_weight: 0.8,
            semantic_weight: 0.2,
        }
    }

    pub fn rerank(
        &self,
        mut results: Vec<SearchResult>,
        graph: &crate::graph::graph::Graph,
    ) -> Vec<SearchResult> {
        let mut semantic = HashMap::<usize, f32>::new();

        for result in &results {
            let neighbors = Traversal::neighborhood(graph, result.document_id(), 1);

            semantic.insert(result.document_id(), neighbors.len() as f32);
        }

        let max = semantic.values().copied().fold(0.0, f32::max).max(1.0);

        for result in &mut results {
            let graph_score = semantic[&result.document_id()] / max;

            let score = self.lexical_weight * result.score() + self.semantic_weight * graph_score;

            result.set_score(score);
        }

        results.sort_by(|a, b| b.score().partial_cmp(&a.score()).unwrap());

        results
    }
}

impl Default for HybridRanker {
    fn default() -> Self {
        Self::new()
    }
}
