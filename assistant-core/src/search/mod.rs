//pub mod aggregator;
pub mod ast;
pub mod document_match;
pub mod engine;
pub mod executor;
pub mod lexer;
pub mod matcher;
pub mod parser;
pub mod parser_error;
pub mod phrase_matcher;
pub mod plan;
pub mod planner;
pub mod ranked_documents;
pub mod result;
pub mod searcher;
pub mod token;

#[cfg(test)]
mod tests;

// -----------------------------------------------------------------------------
// Public API
// -----------------------------------------------------------------------------

pub use ast::QueryNode;
pub use document_match::DocumentMatch;
pub use executor::Executor;
pub use lexer::Lexer;
pub use matcher::Matcher;
pub use parser::Parser;
pub use parser_error::ParserError;
pub use plan::ExecutionPlan;
pub use planner::Planner;
pub use result::SearchResult;
pub use searcher::Searcher;
