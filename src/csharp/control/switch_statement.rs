use crate::csharp::SwitchCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `switch` statement.
pub struct SwitchStatement {
    expression: Box<dyn Expression>,
    switch_cases: Vec<SwitchCase>,
}

impl<E: 'static + Expression> From<E> for SwitchStatement {
    fn from(expression: E) -> Self {
        Self {
            expression: Box::new(expression),
            switch_cases: Vec::default(),
        }
    }
}

impl From<&str> for SwitchStatement {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for SwitchStatement {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl SwitchStatement {
    //! Cases

    /// Gets the switch cases.
    #[must_use]
    pub fn switch_cases(&self) -> &[SwitchCase] {
        self.switch_cases.as_slice()
    }

    /// Adds the switch case.
    pub fn add_switch_case(&mut self, case: SwitchCase) {
        self.switch_cases.push(case);
    }

    /// Adds the switch case.
    #[must_use]
    pub fn with_switch_case(mut self, case: SwitchCase) -> Self {
        self.add_switch_case(case);
        self
    }
}

impl Statement for SwitchStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("switch (");
        self.expression.write(b);
        b.push(')');
        b.end_line();
        b.line(level, "{");
        for switch_case in &self.switch_cases {
            switch_case.write(b, level + 1);
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;

    #[test]
    fn empty_switch() {
        let s: SwitchStatement = SwitchStatement::from("x");
        let mut b: CodeBuffer = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(b.peek(), "    switch (x)\n    {\n    }\n");
    }

    #[test]
    fn switch_from_a_string() {
        let s: SwitchStatement = SwitchStatement::from(String::from("shape.TagNumber"));
        assert_eq!(s.to_code(), "switch (shape.TagNumber)\n{\n}\n");
    }

    #[test]
    fn switch_from_an_expression() {
        let s: SwitchStatement = SwitchStatement::from(Literal::from("shape.TagNumber"));
        assert_eq!(s.to_code(), "switch (shape.TagNumber)\n{\n}\n");
    }

    #[test]
    fn switch_with_cases() {
        let s: SwitchStatement = SwitchStatement::from("x")
            .with_switch_case(SwitchCase::from("0").with_semi("Zero()"))
            .with_switch_case(SwitchCase::default().with_semi("Other()"));
        assert_eq!(s.switch_cases().len(), 2);
        assert_eq!(
            s.to_code(),
            "\
switch (x)
{
    case 0:
        Zero();
        break;
    default:
        Other();
        break;
}
"
        );
    }
}
