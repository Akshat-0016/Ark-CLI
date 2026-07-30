use crate::{
    index::{document::Document, inverted_index::InvertedIndex},
    ranking::{bm25::BM25Ranker, ranker::Ranker},
    search::{
        document_match::DocumentMatch, executor::Executor, lexer::Lexer, parser::Parser,
        planner::Planner, result::SearchResult, searcher::Searcher,
    },
};

pub struct SearchEngine;

impl SearchEngine {
    pub fn search(query: &str, index: &InvertedIndex) -> Vec<SearchResult> {
        let mut lexer = Lexer::new(query);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(_) => return Vec::new(),
        };

        let planner = Planner::new();
        let plan = planner.plan(&ast);

        let searcher = Searcher::new();
        let executor = Executor::new(&searcher, index);

        let docs: Vec<DocumentMatch> = executor.execute(&plan);

        let ranker = BM25Ranker::new();

        ranker.rank(index, &docs)
    }

    pub fn best_documents<'a>(
        query: &str,
        index: &InvertedIndex,
        documents: &'a [Document],
        limit: usize,
    ) -> Vec<(&'a Document, f32)> {
        Self::search(query, index)
            .into_iter()
            .take(limit)
            .filter_map(|result| {
                documents
                    .get(result.document_id())
                    .map(|doc| (doc, result.score()))
            })
            .collect()
    }
}
