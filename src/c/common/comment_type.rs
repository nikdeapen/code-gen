use crate::CodeBuffer;

/// A comment type.
///
/// # Note
/// - A doc comment line can't hold a `*/`, since it ends the comment.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    /// Lines that start with `//`.
    Line,

    /// A `/** ... */` block. A single comment line is written on one line.
    Doc,
}

impl CommentType {
    //! Write

    /// Writes the comment lines.
    pub fn write_lines(&self, b: &mut CodeBuffer, level: usize, lines: &[String]) {
        match self {
            Self::Line => {
                for line in lines {
                    b.indent(level);
                    b.write("//");
                    b.write(line);
                    b.end_line();
                }
            }
            Self::Doc => match lines {
                [] => {}
                [line] => {
                    b.indent(level);
                    b.write("/**");
                    b.write(line);
                    b.write(" */");
                    b.end_line();
                }
                lines => {
                    b.line(level, "/**");
                    for line in lines {
                        b.indent(level);
                        b.write(" *");
                        b.write(line);
                        b.end_line();
                    }
                    b.line(level, " */");
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the comment lines to a string.
    fn write(comment_type: CommentType, level: usize, lines: &[&str]) -> String {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        let mut b: CodeBuffer = CodeBuffer::default();
        comment_type.write_lines(&mut b, level, lines.as_slice());
        b.into()
    }

    #[test]
    fn line_comments() {
        let code: String = write(CommentType::Line, 0, &[" One.", "", " Two."]);
        assert_eq!(code, "// One.\n//\n// Two.\n");
    }

    #[test]
    fn line_comments_indented() {
        let code: String = write(CommentType::Line, 1, &[" One."]);
        assert_eq!(code, "    // One.\n");
    }

    #[test]
    fn doc_comment_without_lines() {
        assert_eq!(write(CommentType::Doc, 0, &[]), "");
    }

    #[test]
    fn doc_comment_with_one_line() {
        let code: String = write(CommentType::Doc, 0, &[" A point."]);
        assert_eq!(code, "/** A point. */\n");
    }

    #[test]
    fn doc_comment_with_lines() {
        let code: String = write(CommentType::Doc, 0, &[" One.", "", " Two."]);
        assert_eq!(code, "/**\n * One.\n *\n * Two.\n */\n");
    }

    #[test]
    fn doc_comment_indented() {
        let code: String = write(CommentType::Doc, 1, &[" One.", " Two."]);
        assert_eq!(code, "    /**\n     * One.\n     * Two.\n     */\n");
    }
}
