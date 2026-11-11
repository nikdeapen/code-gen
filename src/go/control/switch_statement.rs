use crate::go::SwitchCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `switch` statement.
///
/// # Note
/// - A type switch takes its guard as the expression. (ex: `c := shape.(type)`)
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

    /// Gets the cases.
    #[must_use]
    pub fn switch_cases(&self) -> &[SwitchCase] {
        self.switch_cases.as_slice()
    }

    /// Adds the case.
    pub fn add_switch_case(&mut self, case: SwitchCase) {
        self.switch_cases.push(case);
    }

    /// Adds the case.
    #[must_use]
    pub fn with_switch_case(mut self, case: SwitchCase) -> Self {
        self.add_switch_case(case);
        self
    }
}

impl Statement for SwitchStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("switch ");
        self.expression.write(b);
        b.write(" {");
        b.end_line();
        for switch_case in &self.switch_cases {
            switch_case.write(b, level);
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
        let s = SwitchStatement::from("c")
            .with_switch_case(SwitchCase::from("ColorRed").with_literal("return \"Red\""))
            .with_switch_case(SwitchCase::from("ColorGreen").with_literal("return \"Green\""));
        assert_eq!(
            s.to_code(),
            "switch c {\ncase ColorRed:\n    return \"Red\"\ncase ColorGreen:\n    return \"Green\"\n}\n"
        );
    }

    #[test]
    fn switch_with_default() {
        let s = SwitchStatement::from(String::from("c"))
            .with_switch_case(SwitchCase::from("ColorRed").with_literal("return true"))
            .with_switch_case(SwitchCase::default().with_literal("return false"));
        assert_eq!(
            s.to_code(),
            "switch c {\ncase ColorRed:\n    return true\ndefault:\n    return false\n}\n"
        );
    }

    #[test]
    fn switch_with_default_first() {
        let s = SwitchStatement::from("c")
            .with_switch_case(SwitchCase::default().with_literal("return false"))
            .with_switch_case(SwitchCase::from("ColorRed").with_literal("return true"));
        assert_eq!(
            s.to_code(),
            "switch c {\ndefault:\n    return false\ncase ColorRed:\n    return true\n}\n"
        );
    }

    #[test]
    fn type_switch() {
        let s = SwitchStatement::from("c := shape.(type)")
            .with_switch_case(SwitchCase::from("ShapeCircle").with_literal("return c.Value.Area()"))
            .with_switch_case(SwitchCase::default().with_literal("return 0"));
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    switch c := shape.(type) {\n    case ShapeCircle:\n        return c.Value.Area()\n    default:\n        return 0\n    }\n"
        );
    }

    #[test]
    fn switch_without_cases() {
        let s = SwitchStatement::from("c");
        assert_eq!(s.to_code(), "switch c {\n}\n");
    }
}
