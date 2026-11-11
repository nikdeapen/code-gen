use crate::CodeBuffer;

/// A comment type.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    Line,
    LineDoc,
}

impl CommentType {
    //! Write

    /// Writes the comment line.
    pub fn write_line(self, b: &mut CodeBuffer, level: usize, line: &str) {
        b.indent(level);
        b.write(self.delimiter());
        b.write(line);
        b.end_line();
    }

    /// Gets the delimiter.
    const fn delimiter(self) -> &'static str {
        match self {
            Self::Line => "//",
            Self::LineDoc => "///",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line() {
        let mut b = CodeBuffer::default();
        CommentType::Line.write_line(&mut b, 0, " A note.");
        assert_eq!(b.peek(), "// A note.\n");
    }

    #[test]
    fn line_doc() {
        let mut b = CodeBuffer::default();
        CommentType::LineDoc.write_line(&mut b, 1, " A point.");
        assert_eq!(b.peek(), "    /// A point.\n");
    }

    #[test]
    fn empty_line_has_no_trailing_space() {
        let mut b = CodeBuffer::default();
        CommentType::LineDoc.write_line(&mut b, 1, "");
        assert_eq!(b.peek(), "    ///\n");
    }
}
