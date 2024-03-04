use crate::rust::MatchCase;
use crate::{CodeBuffer, Expression, Literal, Statement, WithExpression};

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

impl From<&str> for Match {
    fn from(value: &str) -> Self {
        Self::from(Literal::from(value))
    }
}

impl From<String> for Match {
    fn from(value: String) -> Self {
        Self::from(Literal::from(value))
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
        b.write(" {");
        b.end_line();
        for match_case in self.match_cases() {
            match_case.write(b, level + 1);
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::{Match, MatchCase};
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let match_statement: Match = Match::from("exp");
        let result: String = CodeBuffer::display_statement(&match_statement);
        assert_eq!(result, "match exp {\n}\n");

        let match_statement: Match = match_statement.with_match_case(MatchCase::from("Some(1)"));
        let result: String = CodeBuffer::display_statement(&match_statement);
        assert_eq!(result, "match exp {\n\tSome(1) => {}\n}\n");
    }
}
