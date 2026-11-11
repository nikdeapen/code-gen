use crate::kotlin::WhenCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `when` statement.
///
/// # Note
/// - The default statement has no subject. Each of its cases is a condition.
#[derive(Default)]
pub struct WhenStatement {
    is_return: bool,
    subject: Option<Box<dyn Expression>>,
    when_cases: Vec<WhenCase>,
}

impl<E: 'static + Expression> From<E> for WhenStatement {
    fn from(expression: E) -> Self {
        Self {
            is_return: false,
            subject: Some(Box::new(expression)),
            when_cases: Vec::default(),
        }
    }
}

impl From<&str> for WhenStatement {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for WhenStatement {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl WhenStatement {
    //! Properties

    /// Checks if the statement returns the value of the `when`.
    #[must_use]
    pub fn is_return(&self) -> bool {
        self.is_return
    }

    /// Gets the cases.
    #[must_use]
    pub fn when_cases(&self) -> &[WhenCase] {
        self.when_cases.as_slice()
    }
}

impl WhenStatement {
    //! Return

    /// Makes the statement return the value of the `when`. (ex: `return when (x) {`)
    pub fn set_return(&mut self) {
        self.is_return = true;
    }

    /// Makes the statement return the value of the `when`. (ex: `return when (x) {`)
    #[must_use]
    pub fn with_return(mut self) -> Self {
        self.set_return();
        self
    }
}

impl WhenStatement {
    //! Cases

    /// Adds the case.
    pub fn add_when_case(&mut self, when_case: WhenCase) {
        self.when_cases.push(when_case);
    }

    /// Adds the case.
    #[must_use]
    pub fn with_when_case(mut self, when_case: WhenCase) -> Self {
        self.add_when_case(when_case);
        self
    }
}

impl Statement for WhenStatement {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        if self.is_return {
            b.write("return ");
        }
        b.write("when ");
        if let Some(subject) = &self.subject {
            b.push('(');
            subject.write(b);
            b.write(") ");
        }
        b.push('{');
        b.end_line();
        for when_case in &self.when_cases {
            when_case.write(b, level + 1);
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::kotlin::WithExpression;

    #[test]
    fn when_with_subject() {
        let s = WhenStatement::from("x")
            .with_when_case(WhenCase::from("0").with_literal("zero()"))
            .with_when_case(WhenCase::from("else").with_literal("other()"));
        assert_eq!(
            s.to_code(),
            "when (x) {\n    0 -> {\n        zero()\n    }\n    else -> {\n        other()\n    }\n}\n"
        );
    }

    #[test]
    fn when_without_subject() {
        let s = WhenStatement::default()
            .with_when_case(WhenCase::from("x < 0").with_expression("negative()"))
            .with_when_case(WhenCase::from("else").with_expression("other()"));
        assert_eq!(
            s.to_code(),
            "when {\n    x < 0 -> negative()\n    else -> other()\n}\n"
        );
    }

    #[test]
    fn when_with_return() {
        let s = WhenStatement::from(String::from("x"))
            .with_return()
            .with_when_case(WhenCase::from("0").with_expression("\"zero\""))
            .with_when_case(WhenCase::from("else").with_expression("\"other\""));
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    return when (x) {\n        0 -> \"zero\"\n        else -> \"other\"\n    }\n"
        );
    }

    #[test]
    fn when_from_expression() {
        let s = WhenStatement::from(Literal::from("x"))
            .with_when_case(WhenCase::from("else").with_expression("other()"));
        assert_eq!(s.to_code(), "when (x) {\n    else -> other()\n}\n");
    }
}
