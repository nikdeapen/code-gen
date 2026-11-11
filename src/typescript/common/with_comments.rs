use crate::CodeBuffer;
use crate::typescript::CommentType;

/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comments(&self) -> &[String];

    /// Adds the `comment` line.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Adds the `comment` line.
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
        comment_type.write_lines(b, level, self.comments());
    }
}
