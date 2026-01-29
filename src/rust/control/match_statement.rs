use crate::rust::{MatchCase, Var};
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `match` statement.
pub struct MatchStatement {
    assignment: Option<Var>,
    expression: Box<dyn Expression>,
    match_cases: Vec<MatchCase>,
}

impl<E: 'static + Expression> From<E> for MatchStatement {
    fn from(expression: E) -> Self {
        Self {
            assignment: None,
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
    //! Assignment

    /// Gets the optional variable name assignment.
    pub fn assignment(&self) -> Option<&Var> {
        self.assignment.as_ref()
    }

    /// Sets the variable name `assignment`.
    pub fn set_assignment<V>(&mut self, assignment: V)
    where
        V: Into<Var>,
    {
        self.assignment = Some(assignment.into());
    }

    /// Sets the variable name `assignment`.
    pub fn with_assignment<V>(mut self, assignment: V) -> Self
    where
        V: Into<Var>,
    {
        self.set_assignment(assignment);
        self
    }
}

impl MatchStatement {
    //! Cases

    /// Adds the match `case`.
    pub fn add_match_case(&mut self, case: MatchCase) {
        self.match_cases.push(case);
    }

    /// Adds the match `case`.
    pub fn with_match_case(mut self, case: MatchCase) -> Self {
        self.add_match_case(case);
        self
    }
}

impl Statement for MatchStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        if let Some(assignment) = &self.assignment {
            b.write("let ");
            assignment.write(b);
            b.write(" = ");
        }
        b.write("match ");
        self.expression.write(b);
        b.write(" {");
        b.end_line();
        for match_case in &self.match_cases {
            match_case.write(b, level + 1);
        }
        if self.assignment.is_some() {
            b.line(level, "};");
        } else {
            b.line(level, "}")
        }
    }
}
