use crate::rust::CommentType;
use crate::CodeBuffer;

/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comments.
    fn comments(&self) -> &[String];

    /// Adds the comment.
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }

    /// Adds the comment.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Writes the comments.
    fn write_comments(&self, comment_type: CommentType, b: &mut CodeBuffer, level: usize) {
        for line in self.comments() {
            comment_type.write_line(b, level, line.as_str());
        }
    }
}
