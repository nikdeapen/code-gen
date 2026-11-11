use crate::python::MatchCase;
use crate::{CodeBuffer, Expression, Literal, Statement};

/// A `match` statement.
///
/// # Note
/// - A statement with no cases is written with a `case _` of `pass`, since `match` needs a case.
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
    fn from(expression: &str) -> Self {
        Self::from(Literal::from(expression))
    }
}

impl From<String> for MatchStatement {
    fn from(expression: String) -> Self {
        Self::from(Literal::from(expression))
    }
}

impl MatchStatement {
    //! Cases

    /// Gets the match cases.
    #[must_use]
    pub fn match_cases(&self) -> &[MatchCase] {
        self.match_cases.as_slice()
    }

    /// Adds a match case.
    pub fn add_match_case(&mut self, case: MatchCase) {
        self.match_cases.push(case);
    }

    /// Adds a match case.
    #[must_use]
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
        b.push(':');
        b.end_line();
        if self.match_cases.is_empty() {
            MatchCase::from("_").write(b, level + 1);
        } else {
            for match_case in &self.match_cases {
                match_case.write(b, level + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;

    #[test]
    fn simple_match() {
        let s = MatchStatement::from("x")
            .with_match_case(MatchCase::from("0").with_literal("zero()"))
            .with_match_case(MatchCase::from("_").with_literal("other()"));
        assert_eq!(
            s.to_code(),
            "match x:\n    case 0:\n        zero()\n    case _:\n        other()\n"
        );
    }

    #[test]
    fn empty_match() {
        let s = MatchStatement::from(String::from("x"));
        assert_eq!(s.to_code(), "match x:\n    case _:\n        pass\n");
    }

    #[test]
    fn match_indented() {
        let s =
            MatchStatement::from("x").with_match_case(MatchCase::from("0").with_literal("zero()"));
        let mut b = CodeBuffer::default();
        s.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    match x:\n        case 0:\n            zero()\n"
        );
    }
}
