use crate::CodeBuffer;

/// A comment type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    Line,
    InnerLineDoc,
    OuterLineDoc,
}

impl CommentType {
    //! Write

    /// Gets the delimiter.
    const fn delimiter(&self) -> &str {
        match self {
            Self::Line => "//",
            Self::InnerLineDoc => "//!",
            Self::OuterLineDoc => "///",
        }
    }

    /// Writes the comment `line`.
    pub fn write_line(&self, b: &mut CodeBuffer, level: usize, line: &str) {
        b.indent(level);
        b.write(self.delimiter());
        b.write(line);
        b.end_line();
    }
}
