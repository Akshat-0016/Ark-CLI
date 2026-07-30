//! Legacy query type.
//
// This module exists only as a temporary compatibility layer while
// the search engine is migrated to the AST pipeline.
//
// New code MUST NOT use this type.
//
// Engine -> Lexer -> Parser -> AST -> Evaluator
//
// Once every legacy strategy has been removed,
// this file can be deleted.

#[deprecated(note = "Use the Lexer + Parser + AST pipeline instead.")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryOperator {
    And,
    Or,
    Not,
}

#[deprecated(note = "Use the AST pipeline instead.")]
#[derive(Debug, Clone)]
pub struct Query {
    terms: Vec<String>,
    operator: QueryOperator,
    phrase: bool,
}

impl Query {
    #[deprecated(note = "Use Engine::search(&str) instead.")]
    pub fn new(query: impl Into<String>) -> Self {
        let mut input = query.into().trim().to_string();

        let phrase = input.starts_with('"') && input.ends_with('"');

        if phrase {
            input = input.trim_matches('"').to_string();
        }

        let input = input.to_lowercase();

        let mut operator = QueryOperator::Or;
        let mut terms = Vec::new();

        for token in input.split_whitespace() {
            match token {
                "and" => operator = QueryOperator::And,
                "or" => operator = QueryOperator::Or,
                "not" => operator = QueryOperator::Not,
                _ => terms.push(token.to_string()),
            }
        }

        Self {
            terms,
            operator,
            phrase,
        }
    }

    pub fn terms(&self) -> &[String] {
        &self.terms
    }

    pub fn operator(&self) -> QueryOperator {
        self.operator
    }

    pub fn is_phrase(&self) -> bool {
        self.phrase
    }

    #[deprecated(note = "Legacy compatibility only.")]
    pub fn with_operator(&self, operator: QueryOperator) -> Self {
        Self {
            terms: self.terms.clone(),
            operator,
            phrase: self.phrase,
        }
    }
}
