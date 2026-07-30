#[derive(Debug, Clone)]
pub enum QueryNode {
    Term(String),

    Phrase(String),

    And(Box<QueryNode>, Box<QueryNode>),

    Or(Box<QueryNode>, Box<QueryNode>),

    Not(Box<QueryNode>),
}
