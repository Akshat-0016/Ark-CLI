pub mod ai;
pub mod cache;
pub mod config;
pub mod file_index;
pub mod file_router;
pub mod index;
pub mod info_dump;
pub mod intent;
pub mod memory;
pub mod nlp;
pub mod pdf;
pub mod query;
pub mod ranking;
pub mod retrieval;
pub mod router;
pub mod search;
pub mod skills;
pub mod utils;

use crate::{
    index::{document::Document, inverted_index::InvertedIndex},
    memory::Memory,
};

pub async fn ask_engine(
    input: String,
    index: &InvertedIndex,
    documents: &[Document],
    memory: &mut Memory,
) -> String {
    router::route(&input, index, documents, memory).await
}
