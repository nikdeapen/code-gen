use crate::python::{PythonType, WithComments};
use crate::{CodeBuffer, Expression, Statement, WithName};

/// A type alias declaration. (ex: `type Shape = Circle | Square`)
///
/// # Note
/// - Its comment lines are a docstring on the line after it.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TypeDec {
    name: String,
    python_type: PythonType,
    comments: Vec<String>,
}

impl<S: Into<String>, T: Into<PythonType>> From<(S, T)> for TypeDec {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            python_type: tuple.1.into(),
            comments: Vec::default(),
        }
    }
}

impl WithName for TypeDec {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl TypeDec {
    //! Type

    /// Gets the type.
    #[must_use]
    pub fn python_type(&self) -> &PythonType {
        &self.python_type
    }
}

impl WithComments for TypeDec {
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

impl Statement for TypeDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("type ");
        self.write_name(b);
        b.write(" = ");
        self.python_type.write(b);
        b.end_line();
        self.write_comments(b, level);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_dec() {
        let t = TypeDec::from(("Id", "int"));
        assert_eq!(t.to_code(), "type Id = int\n");
    }

    #[test]
    fn type_dec_with_comment() {
        let shape = PythonType::Union(vec!["Circle".into(), "Square".into()]);
        let t = TypeDec::from(("Shape", shape)).with_comment("A shape.");
        assert_eq!(
            t.to_code(),
            "type Shape = Circle | Square\n\"\"\"A shape.\"\"\"\n"
        );
    }

    #[test]
    fn type_dec_indented() {
        let t = TypeDec::from(("Id", "int")).with_comment("An id.");
        let mut b = CodeBuffer::default();
        t.write(&mut b, 1);
        assert_eq!(b.peek(), "    type Id = int\n    \"\"\"An id.\"\"\"\n");
    }
}
