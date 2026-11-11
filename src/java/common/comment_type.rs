use crate::CodeBuffer;

/// A comment type.
///
/// # Note
/// - A line is written right after its delimiter. Start it with a space.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    /// Lines that each start with `//`.
    Line,

    /// A Javadoc block.
    Doc,
}

impl CommentType {
    //! Write

    /// Writes the lines of a comment.
    pub fn write_lines(&self, b: &mut CodeBuffer, level: usize, lines: &[String]) {
        match self {
            Self::Line => {
                for line in lines {
                    self.write_line(b, level, "//", line);
                }
            }
            Self::Doc => {
                if !lines.is_empty() {
                    b.line(level, "/**");
                    for line in lines {
                        self.write_line(b, level, " *", line);
                    }
                    b.line(level, " */");
                }
            }
        }
    }

    /// Writes a line of a comment after its delimiter.
    fn write_line(&self, b: &mut CodeBuffer, level: usize, delimiter: &str, line: &str) {
        b.indent(level);
        b.write(delimiter);
        b.write(line);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the `lines` as the `comment_type` at indent level 1.
    fn write(comment_type: CommentType, lines: &[&str]) -> String {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        let mut b: CodeBuffer = CodeBuffer::default();
        comment_type.write_lines(&mut b, 1, &lines);
        b.into()
    }

    #[test]
    fn line_comments() {
        assert_eq!(
            write(CommentType::Line, &[" One.", "", " Two."]),
            "    // One.\n    //\n    // Two.\n"
        );
    }

    #[test]
    fn no_line_comments() {
        assert_eq!(write(CommentType::Line, &[]), "");
    }

    #[test]
    fn doc_comments() {
        assert_eq!(
            write(CommentType::Doc, &[" One.", "", " Two."]),
            "    /**\n     * One.\n     *\n     * Two.\n     */\n"
        );
    }

    #[test]
    fn no_doc_comments() {
        assert_eq!(write(CommentType::Doc, &[]), "");
    }
}
