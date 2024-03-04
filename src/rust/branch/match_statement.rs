use crate::rust::MatchCase;
use crate::{CodeBuffer, Expression, Statement, WithExpression};

/// A match statement.
pub struct Match {
    expression: Box<dyn Expression>,
    match_cases: Vec<MatchCase>,
}

impl<E: 'static + Expression> From<E> for Match {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
            match_cases: Vec::default(),
        }
    }
}

impl WithExpression for Match {
    fn expression(&self) -> &Box<dyn Expression> {
        &self.expression
    }
}

impl Match {
    //! Match Cases

    /// Gets the match cases.
    pub fn match_cases(&self) -> &[MatchCase] {
        self.match_cases.as_slice()
    }

    /// Adds the match case.
    pub fn add_match_case<MC>(&mut self, match_case: MC)
    where
        MC: Into<MatchCase>,
    {
        self.match_cases.push(match_case.into());
    }

    /// Adds the match case.
    pub fn with_match_case<MC>(mut self, match_case: MC) -> Self
    where
        MC: Into<MatchCase>,
    {
        self.add_match_case(match_case);
        self
    }
}

impl Statement for Match {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("match ");
        self.expression.write(b);
        b.write("{");
        b.end_line();
        for match_case in self.match_cases() {
            match_case.write(b, level + 1);
        }
        b.line(level, "}");
    }
}
