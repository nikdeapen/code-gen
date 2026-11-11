use crate::java::SwitchCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `switch` statement, or a `switch` expression that is returned.
pub struct SwitchStatement {
    is_return: bool,
    subject: Box<dyn Expression>,
    switch_cases: Vec<SwitchCase>,
}

impl<E: 'static + Expression> From<E> for SwitchStatement {
    fn from(subject: E) -> Self {
        Self {
            is_return: false,
            subject: Box::new(subject),
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
    //! Return

    /// Checks if the switch is an expression that is returned.
    #[must_use]
    pub fn is_return(&self) -> bool {
        self.is_return
    }

    /// Sets if the switch is an expression that is returned.
    pub fn set_return(&mut self, is_return: bool) {
        self.is_return = is_return;
    }

    /// Sets if the switch is an expression that is returned.
    #[must_use]
    pub fn with_return(mut self, is_return: bool) -> Self {
        self.set_return(is_return);
        self
    }
}

impl SwitchStatement {
    //! Cases

    /// Gets the cases.
    #[must_use]
    pub fn switch_cases(&self) -> &[SwitchCase] {
        self.switch_cases.as_slice()
    }

    /// Adds a case.
    pub fn add_switch_case(&mut self, case: SwitchCase) {
        self.switch_cases.push(case);
    }

    /// Adds a case.
    #[must_use]
    pub fn with_switch_case(mut self, case: SwitchCase) -> Self {
        self.add_switch_case(case);
        self
    }
}

impl Statement for SwitchStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        if self.is_return {
            b.write("return ");
        }
        b.write("switch (");
        self.subject.write(b);
        b.write(") {");
        b.end_line();
        for switch_case in &self.switch_cases {
            switch_case.write(b, level + 1);
        }
        if self.is_return {
            b.line(level, "};");
        } else {
            b.line(level, "}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;

    #[test]
    fn simple_switch() {
        let s: SwitchStatement = SwitchStatement::from("x")
            .with_switch_case(SwitchCase::from("0").with_semi("zero()"))
            .with_switch_case(SwitchCase::default().with_semi("other()"));
        assert_eq!(
            s.to_code(),
            "switch (x) {\n    case 0 -> {\n        zero();\n    }\n    default -> {\n        other();\n    }\n}\n"
        );
    }

    #[test]
    fn returned_switch() {
        let s: SwitchStatement = SwitchStatement::from(String::from("tagNumber"))
            .with_return(true)
            .with_switch_case(SwitchCase::from("1").with_expression("RED"))
            .with_switch_case(SwitchCase::default().with_expression("UNRECOGNIZED"));
        assert_eq!(
            s.to_code(),
            "return switch (tagNumber) {\n    case 1 -> RED;\n    default -> UNRECOGNIZED;\n};\n"
        );
    }

    #[test]
    fn switch_from_an_expression() {
        let s: SwitchStatement = SwitchStatement::from(Literal::from("x"))
            .with_switch_case(SwitchCase::from("0").with_expression("zero()"));
        let mut b: CodeBuffer = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    switch (x) {\n        case 0 -> zero();\n    }\n"
        );
    }
}
