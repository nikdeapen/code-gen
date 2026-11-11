use crate::cpp::{CommentType, WithComments};
use crate::{CodeBuffer, Statement, WithName};

/// An enum case. (ex: `Red = 1,`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct EnumCase {
    comments: Vec<String>,
    name: String,
    value: Option<String>,
}

impl<S: Into<String>> From<S> for EnumCase {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            name: name.into(),
            value: None,
        }
    }
}

impl WithComments for EnumCase {
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

impl WithName for EnumCase {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl EnumCase {
    //! Value

    /// Gets the optional value.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Sets the value.
    pub fn set_value<S>(&mut self, value: S)
    where
        S: Into<String>,
    {
        self.value = Some(value.into());
    }

    /// Sets the value.
    #[must_use]
    pub fn with_value<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.set_value(value);
        self
    }
}

impl Statement for EnumCase {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        self.write_name(b);
        if let Some(value) = &self.value {
            b.write(" = ");
            b.write(value.as_str());
        }
        b.push(',');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let c = EnumCase::from("Red");
        assert_eq!(c.value(), None);
        assert_eq!(c.to_code(), "Red,\n");
    }

    #[test]
    fn case_with_value() {
        let c = EnumCase::from("Red").with_value("1");
        assert_eq!(c.value(), Some("1"));
        assert_eq!(c.to_code(), "Red = 1,\n");
    }

    #[test]
    fn case_with_comment() {
        let c = EnumCase::from("Red")
            .with_comment(" The color red.")
            .with_value("1");
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(b.peek(), "    /// The color red.\n    Red = 1,\n");
    }
}
