use crate::cpp::SwitchCase;
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
        b.write(") {");
        b.end_line();
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
    fn simple_switch() {
        let s = SwitchStatement::from("x")
            .with_switch_case(SwitchCase::from("0").with_semi("zero()"))
            .with_switch_case(SwitchCase::from("1").with_semi("one()"));
        assert_eq!(s.switch_cases().len(), 2);
        assert_eq!(
            s.to_code(),
            "switch (x) {\n    case 0: {\n        zero();\n        break;\n    }\n    case 1: {\n        one();\n        break;\n    }\n}\n"
        );
    }

    #[test]
    fn switch_with_default_case() {
        let s = SwitchStatement::from(String::from("color"))
            .with_switch_case(
                SwitchCase::from("Color::Red")
                    .with_semi("return 1")
                    .with_break(false),
            )
            .with_switch_case(
                SwitchCase::default()
                    .with_semi("return 0")
                    .with_break(false),
            );
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    switch (color) {\n        case Color::Red: {\n            return 1;\n        }\n        default: {\n            return 0;\n        }\n    }\n"
        );
    }

    #[test]
    fn default_case_is_written_where_it_was_added() {
        let s = SwitchStatement::from("x")
            .with_switch_case(SwitchCase::default().with_semi("other()"))
            .with_switch_case(SwitchCase::from("0").with_semi("zero()"));
        assert_eq!(
            s.to_code(),
            "switch (x) {\n    default: {\n        other();\n        break;\n    }\n    case 0: {\n        zero();\n        break;\n    }\n}\n"
        );
    }

    #[test]
    fn empty_switch() {
        let s = SwitchStatement::from("x");
        assert_eq!(s.to_code(), "switch (x) {\n}\n");
    }
}
