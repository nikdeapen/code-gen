use crate::CodeBuffer;

/// A comment type.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    /// A line comment. (`//`)
    Line,

    /// A KDoc comment. (`/**`)
    Doc,
}

impl CommentType {
    //! Write

    /// Writes the comment lines.
    ///
    /// # Note
    /// - A line is written right after its delimiter. Start a line of text with a space.
    /// - A KDoc comment with no lines is not written.
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

    /// Writes the comment `line` after its `delimiter`.
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

    /// Writes the `lines` as the `comment_type` at the indent `level`.
    fn code(comment_type: CommentType, level: usize, lines: &[&str]) -> String {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        let mut b = CodeBuffer::default();
        comment_type.write_lines(&mut b, level, &lines);
        b.into()
    }

    #[test]
    fn line_comment() {
        assert_eq!(
            code(
                CommentType::Line,
                0,
                &[" Gets the name.", "", " Never empty."]
            ),
            "// Gets the name.\n//\n// Never empty.\n"
        );
    }

    #[test]
    fn line_comment_without_lines() {
        assert_eq!(code(CommentType::Line, 0, &[]), "");
    }

    #[test]
    fn doc_comment() {
        assert_eq!(
            code(
                CommentType::Doc,
                0,
                &[" Gets the name.", "", " Never empty."]
            ),
            "/**\n * Gets the name.\n *\n * Never empty.\n */\n"
        );
    }

    #[test]
    fn doc_comment_indented() {
        assert_eq!(
            code(CommentType::Doc, 1, &[" Gets the name."]),
            "    /**\n     * Gets the name.\n     */\n"
        );
    }

    #[test]
    fn doc_comment_without_lines() {
        assert_eq!(code(CommentType::Doc, 0, &[]), "");
    }
}
