use crate::CodeBuffer;

/// A comment type.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    /// Line comments. (ex: `// text`)
    Line,

    /// A JSDoc comment. (ex: `/** text */`)
    Doc,
}

impl CommentType {
    //! Write

    /// Writes the comment `lines`.
    ///
    /// # Note
    /// - No space is written between a delimiter & its line.
    /// - A JSDoc comment of one line is written as one line.
    pub fn write_lines(&self, b: &mut CodeBuffer, level: usize, lines: &[String]) {
        match (self, lines) {
            (Self::Line, lines) => {
                for line in lines {
                    b.indent(level);
                    b.write("//");
                    b.write(line);
                    b.end_line();
                }
            }
            (Self::Doc, []) => {}
            (Self::Doc, [line]) => {
                b.indent(level);
                b.write("/**");
                b.write(line);
                b.write(" */");
                b.end_line();
            }
            (Self::Doc, lines) => {
                b.line(level, "/**");
                for line in lines {
                    b.indent(level);
                    b.write(" *");
                    b.write(line);
                    b.end_line();
                }
                b.line(level, " */");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Writes the `lines` as the `comment_type` at the indent `level`.
    fn write(comment_type: CommentType, level: usize, lines: &[&str]) -> String {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        let mut b: CodeBuffer = CodeBuffer::default();
        comment_type.write_lines(&mut b, level, &lines);
        b.into()
    }

    #[test]
    fn line_comments() {
        assert_eq!(write(CommentType::Line, 0, &[]), "");
        assert_eq!(
            write(CommentType::Line, 1, &[" One.", "", " Two."]),
            "    // One.\n    //\n    // Two.\n"
        );
    }

    #[test]
    fn doc_comment_of_no_lines() {
        assert_eq!(write(CommentType::Doc, 0, &[]), "");
    }

    #[test]
    fn doc_comment_of_one_line() {
        assert_eq!(
            write(CommentType::Doc, 1, &[" A point."]),
            "    /** A point. */\n"
        );
    }

    #[test]
    fn doc_comment_of_many_lines() {
        assert_eq!(
            write(CommentType::Doc, 1, &[" A point.", "", " It has no size."]),
            "    /**\n     * A point.\n     *\n     * It has no size.\n     */\n"
        );
    }
}
