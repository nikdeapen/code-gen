use crate::rust::MatchCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `match` statement.
pub struct MatchStatement {
    expression: Box<dyn Expression>,
    match_cases: Vec<MatchCase>,
}

impl<E: 'static + Expression> From<E> for MatchStatement {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
            match_cases: Vec::default(),
        }
    }
}

impl From<&str> for MatchStatement {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for MatchStatement {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl MatchStatement {
    //! Cases

    /// Adds the match case.
    pub fn add_match_case(&mut self, case: MatchCase) {
        self.match_cases.push(case);
    }

    /// Adds the case.
    pub fn with_match_case(mut self, case: MatchCase) -> Self {
        self.add_match_case(case);
        self
    }
}

impl Statement for MatchStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("match ");
        self.expression.write(b);
        b.write(" {");
        b.end_line();
        for match_case in &self.match_cases {
            match_case.write(b, level + 1);
        }
        b.line(level, "}")
    }
}
