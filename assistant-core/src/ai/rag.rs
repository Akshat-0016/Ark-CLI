use std::collections::{HashMap, HashSet};

use crate::{
    index::document::Document,
    nlp,
    nlp::{normalizer::pdf::PdfNormalizer, sentence::splitter::SentenceSplitter},
};
pub struct Rag;

impl Rag {
    const CHUNK_SIZE: usize = 1000;
    const MAX_CHUNKS_PER_DOC: usize = 3;
    const MIN_SCORE: usize = 1;
    const TITLE_BOOST: usize = 50;
    const EXACT_QUERY_BOOST: usize = 100;
    const BM25_WEIGHT: f32 = 10.0;

    pub fn summarize(query: &str, docs: &[(&Document, f32)]) -> String {
        let keywords = nlp::process_words(query).unwrap_or_default();
        let query_lower = query.to_lowercase();

        let mut output = String::new();

        output.push_str(&format!("Query: {}\n\n", query));

        for (i, (doc, bm25)) in docs.iter().enumerate() {
            let cleaned = PdfNormalizer::normalize(doc.text());

            let title_words: HashSet<String> = nlp::process_words(doc.title())
                .unwrap_or_default()
                .into_iter()
                .collect();

            let mut scored: Vec<(usize, String)> =
                SentenceSplitter::chunk(&cleaned, Self::CHUNK_SIZE)
                    .into_iter()
                    .map(|chunk| {
                        let mut frequencies = HashMap::new();

                        for word in nlp::process_words(&chunk).unwrap_or_default() {
                            *frequencies.entry(word).or_insert(0usize) += 1;
                        }

                        let mut score = (*bm25 * Self::BM25_WEIGHT) as usize;

                        // Keyword overlap
                        for keyword in &keywords {
                            score += frequencies.get(keyword).copied().unwrap_or(0);
                        }

                        // Exact query boost
                        let chunk_lower = chunk.to_lowercase();

                        if chunk_lower.contains(&query_lower) {
                            score += Self::EXACT_QUERY_BOOST;
                        }

                        // Title boost
                        score += keywords.iter().filter(|k| title_words.contains(*k)).count()
                            * Self::TITLE_BOOST;

                        (score, chunk)
                    })
                    .filter(|(score, _)| *score >= Self::MIN_SCORE)
                    .collect();

            if scored.is_empty() {
                continue;
            }

            scored.sort_by(|a, b| b.0.cmp(&a.0));

            output.push_str(&format!("===== SOURCE {} =====\n", i + 1));
            output.push_str(&format!("Title: {}\n", doc.title()));
            output.push_str(&format!("Path: {}\n", doc.path()));
            output.push('\n');

            for (score, chunk) in scored.iter().take(Self::MAX_CHUNKS_PER_DOC) {
                output.push_str(&format!("[score: {}]\n", score));
                output.push_str(chunk.trim());
                output.push_str("\n\n");
            }
        }

        if output.trim() == format!("Query: {}", query).trim() {
            output.push_str("No relevant information found.");
        }

        output
    }
}
