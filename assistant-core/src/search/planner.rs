use super::{ast::QueryNode, plan::ExecutionPlan};
pub struct Planner;

impl Planner {
    pub fn new() -> Self {
        Self
    }

    /// Produce an execution plan from an AST.
    pub fn plan(&self, root: &QueryNode) -> ExecutionPlan {
        self.build(root)
    }

    fn build(&self, node: &QueryNode) -> ExecutionPlan {
        match node {
            QueryNode::Term(term) => ExecutionPlan::term(term.clone()),

            QueryNode::Phrase(phrase) => {
                let terms = phrase
                    .split_whitespace()
                    .map(str::to_owned)
                    .collect::<Vec<_>>();

                ExecutionPlan::phrase(terms)
            }

            QueryNode::And(left, right) => {
                ExecutionPlan::intersect(self.build(left), self.build(right))
            }

            QueryNode::Or(left, right) => ExecutionPlan::union(self.build(left), self.build(right)),

            QueryNode::Not(node) => ExecutionPlan::difference(self.build(node)),
        }
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}
