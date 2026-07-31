use crate::index::document::Document;
use crate::pdf::read_pdf;
use crate::{
    index::{build_artifacts::BuildArtifacts, inverted_index::InvertedIndex},
    nlp::{
        normalizer::normalizer::Normalizer,
        pipeline::{PipelineStage, TokenizerStage},
        tokenizer::{token::TokenKind, tokenizer::Tokenizer},
    },
};
use std::fs;
//use std::path::Path;

/// Builds every indexing artifact required by the search engine.
///
/// Responsibilities:
/// - Clean Markdown
/// - Tokenize text
/// - Normalize tokens
/// - Build the inverted index
/// - Extract semantic metadata
///
/// Responsibilities intentionally NOT included:
/// - Graph construction
/// - Embeddings
/// - Ranking
/// - Knowledge engine
pub struct IndexBuilder {
    tokenizer: Tokenizer,
    normalizer: Normalizer,
}

impl IndexBuilder {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            normalizer: Normalizer::new(),
        }
    }

    /// Builds every searchable artifact for a vault.
    pub fn build_from_folder(&self, folder: &str) -> BuildArtifacts {
        let mut index = InvertedIndex::new();
        let mut documents = Vec::new();

        let mut document_id = 0;

        for entry in fs::read_dir(folder).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("pdf") {
                continue;
            }

            let path_string = path.to_string_lossy().to_string();

            if let Some(text) = read_pdf(&path_string) {
                self.index_document(&mut index, document_id, &text);

                let title = path.file_stem().unwrap().to_string_lossy().to_string();

                documents.push(Document::new(document_id, title, path_string, text));

                document_id += 1;
            }
        }

        BuildArtifacts::new(index, documents)
    }

    fn index_document(&self, index: &mut InvertedIndex, document_id: usize, text: &str) {
        let tokens = match self.token_stream(text) {
            Ok(tokens) => tokens,
            Err(_) => return,
        };

        let document_length = tokens
            .iter()
            .filter(|token| token.kind() == TokenKind::Word)
            .count();

        index.add_document(document_length);

        let mut position = 0u32;

        for token in tokens {
            if token.kind() != TokenKind::Word {
                continue;
            }

            index.insert(token.lexeme(), document_id, position);

            position += 1;
        }
    }

    fn token_stream(
        &self,
        text: &str,
    ) -> Result<
        Vec<crate::nlp::tokenizer::token::Token>,
        crate::nlp::tokenizer::error::TokenizerError,
    > {
        let tokens = self.tokenizer.tokenize(text)?;

        Ok(self.normalizer.process(tokens))
    }
}

impl Default for IndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}
