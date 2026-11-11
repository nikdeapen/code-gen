use crate::CodeBuffer;
use crate::cpp::CommentType;

/// An element with comment lines.
///
/// # Note
/// - A line is written right after its delimiter. It starts with its own space.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comments(&self) -> &[String];

    /// Adds the comment line.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Adds the comment line.
    #[must_use]
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }

    /// Writes the comment lines.
    fn write_comments(&self, comment_type: CommentType, b: &mut CodeBuffer, level: usize) {
        for line in self.comments() {
            comment_type.write_line(b, level, line.as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::Field;

    #[test]
    fn write_comments() {
        let field = Field::from(("x", "int"))
            .with_comment(" The x coordinate.")
            .with_comment("")
            .with_comment(" It is never negative.");
        let mut b = CodeBuffer::default();
        field.write_comments(CommentType::LineDoc, &mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// The x coordinate.\n    ///\n    /// It is never negative.\n"
        );
    }
}
