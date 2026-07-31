use crate::ai::rag::Rag;
use crate::cache::{get_cached, set_cached};
//use crate::config::load_config;
use crate::index::{document::Document, inverted_index::InvertedIndex};
use crate::memory::Memory;
use crate::search::engine::SearchEngine;
use crate::skills::system;

pub async fn route(
    input: &str,
    index: &InvertedIndex,
    documents: &[Document],
    memory: &mut Memory,
) -> String {
    memory.remember(input);

    if input.starts_with("system") {
        return system::handle().await;
    }

    if let Some(answer) = get_cached(input) {
        return answer;
    }

    let matches = SearchEngine::best_documents(input, index, documents, 3);

    println!("Retrieved {} documents:", matches.len());

    for (doc, score) in &matches {
        println!(" - {} ({:.2})", doc.path(), score);
    }

    let context = Rag::summarize(input, &matches);

    set_cached(input, context.clone());

    context
}
