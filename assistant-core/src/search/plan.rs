#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionPlan {
    /// Search for a single term.
    Term(String),

    /// Search for an exact phrase.
    Phrase(Vec<String>),

    /// Logical intersection.
    Intersect(Box<ExecutionPlan>, Box<ExecutionPlan>),

    /// Logical union.
    Union(Box<ExecutionPlan>, Box<ExecutionPlan>),

    /// Logical difference.
    Difference(Box<ExecutionPlan>),
}

impl ExecutionPlan {
    pub fn term(term: impl Into<String>) -> Self {
        Self::Term(term.into())
    }

    pub fn phrase<T>(terms: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<String>,
    {
        Self::Phrase(terms.into_iter().map(Into::into).collect())
    }

    pub fn intersect(left: ExecutionPlan, right: ExecutionPlan) -> Self {
        Self::Intersect(Box::new(left), Box::new(right))
    }

    pub fn union(left: ExecutionPlan, right: ExecutionPlan) -> Self {
        Self::Union(Box::new(left), Box::new(right))
    }

    pub fn difference(plan: ExecutionPlan) -> Self {
        Self::Difference(Box::new(plan))
    }

    pub fn is_leaf(&self) -> bool {
        matches!(self, Self::Term(_) | Self::Phrase(_))
    }
}
