use crate::c::{CommentType, WithComments};
use crate::{CodeBuffer, Statement, WithName};

/// A `#define` of a constant.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Define {
    comments: Vec<String>,
    name: String,
    value: String,
}

impl<S0: Into<String>, S1: Into<String>> From<(S0, S1)> for Define {
    fn from(tuple: (S0, S1)) -> Self {
        Self {
            comments: Vec::default(),
            name: tuple.0.into(),
            value: tuple.1.into(),
        }
    }
}

impl WithComments for Define {
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

impl WithName for Define {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Define {
    //! Value

    /// Gets the value. (ex: the `8` of `#define POINT_ENCODED_LEN 8`)
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

impl Statement for Define {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        b.indent(level);
        b.write("#define ");
        self.write_name(b);
        if !self.value.is_empty() {
            b.space();
            b.write(self.value.as_str());
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_define() {
        let d: Define = Define::from(("POINT_ENCODED_LEN", "8"));
        assert_eq!(d.value(), "8");
        assert_eq!(d.to_code(), "#define POINT_ENCODED_LEN 8\n");
    }

    #[test]
    fn define_without_value() {
        let d: Define = Define::from(("POINT_DEBUG", ""));
        assert_eq!(d.to_code(), "#define POINT_DEBUG\n");
    }

    #[test]
    fn define_with_comment() {
        let d: Define = Define::from(("POINT_ENCODED_LEN", "8")).with_comment(" The length.");
        assert_eq!(
            d.to_code(),
            "/** The length. */\n#define POINT_ENCODED_LEN 8\n"
        );
    }

    #[test]
    fn define_indented() {
        let d: Define = Define::from(("POINT_ENCODED_LEN", "8")).with_comment(" The length.");
        let mut b: CodeBuffer = CodeBuffer::default();
        d.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /** The length. */\n    #define POINT_ENCODED_LEN 8\n"
        );
    }
}
