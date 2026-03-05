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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn with_match_case(mut self, case: MatchCase) -> Self {
        self.add_match_case(case);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;

    #[test]
    fn simple_match() {
        let s = MatchStatement::from("x")
            .with_match_case(MatchCase::from("0").with_semi("zero()"))
            .with_match_case(MatchCase::from("_").with_semi("other()"));
        assert_eq!(
            s.to_code(),
            "match x {\n    0 => {\n        zero();\n    }\n    _ => {\n        other();\n    }\n}\n"
        );
    }

    #[test]
    fn match_with_assignment() {
        let s = MatchStatement::from("x")
            .with_assignment(Var::from(("result", "u32")))
            .with_match_case(MatchCase::from("_").with_semi("0"));
        assert_eq!(
            s.to_code(),
            "let result: u32 = match x {\n    _ => {\n        0;\n    }\n};\n"
        );
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
            b.line(level, "}");
        }
    }
}
