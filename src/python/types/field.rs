use crate::python::{Var, WithComments};
use crate::{CodeBuffer, Expression, Statement};

/// A field of a class. (ex: `x: int` or `RED = 1`)
///
/// # Note
/// - Its comment lines are a docstring on the line after it.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Field {
    var: Var,
    comments: Vec<String>,
}

impl<V: Into<Var>> From<V> for Field {
    fn from(var: V) -> Self {
        Self {
            var: var.into(),
            comments: Vec::default(),
        }
    }
}

impl Field {
    //! Var

    /// Gets the variable.
    #[must_use]
    pub fn var(&self) -> &Var {
        &self.var
    }
}

impl WithComments for Field {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl Statement for Field {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        self.var.write(b);
        b.end_line();
        self.write_comments(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_with_type() {
        let f = Field::from(("x", "int"));
        assert_eq!(f.to_code(), "x: int\n");
    }

    #[test]
    fn field_with_value() {
        let f = Field::from(Var::from("RED").with_value("1"));
        assert_eq!(f.to_code(), "RED = 1\n");
    }

    #[test]
    fn field_with_comment() {
        let f = Field::from(("x", "int")).with_comment("The x.");
        assert_eq!(f.to_code(), "x: int\n\"\"\"The x.\"\"\"\n");
    }

    #[test]
    fn field_indented() {
        let f = Field::from(Var::from(("x", "int")).with_value("0")).with_comment("The x.");
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(b.peek(), "    x: int = 0\n    \"\"\"The x.\"\"\"\n");
    }
}
