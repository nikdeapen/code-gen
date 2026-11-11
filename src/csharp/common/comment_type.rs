use crate::CodeBuffer;

/// A comment type.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CommentType {
    Line,

    /// An XML doc comment. Its lines are the text of the `<summary>` element.
    Doc,
}

impl CommentType {
    //! Write

    /// Writes the comment lines.
    ///
    /// # Note
    /// - Each line is written right after the delimiter. A line of text starts with a space.
    pub fn write_lines(&self, b: &mut CodeBuffer, level: usize, lines: &[String]) {
        if lines.is_empty() {
            return;
        }
        if *self == Self::Doc {
            self.write_line(b, level, " <summary>");
        }
        for line in lines {
            self.write_line(b, level, line.as_str());
        }
        if *self == Self::Doc {
            self.write_line(b, level, " </summary>");
        }
    }

    /// Writes the comment line.
    fn write_line(&self, b: &mut CodeBuffer, level: usize, line: &str) {
        b.indent(level);
        b.write(self.delimiter());
        b.write(line);
        b.end_line();
    }

    /// Gets the delimiter.
    const fn delimiter(&self) -> &str {
        match self {
            Self::Line => "//",
            Self::Doc => "///",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Gets the code of the comment `lines`.
    fn code(comment_type: CommentType, level: usize, lines: &[&str]) -> String {
        let lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        let mut b: CodeBuffer = CodeBuffer::default();
        comment_type.write_lines(&mut b, level, lines.as_slice());
        b.into()
    }

    #[test]
    fn no_lines() {
        assert_eq!(code(CommentType::Line, 0, &[]), "");
        assert_eq!(code(CommentType::Doc, 0, &[]), "");
    }

    #[test]
    fn line_comment() {
        assert_eq!(
            code(CommentType::Line, 0, &[" One.", " Two."]),
            "// One.\n// Two.\n"
        );
    }

    #[test]
    fn doc_comment() {
        assert_eq!(
            code(CommentType::Doc, 0, &[" One.", "", " Two."]),
            "/// <summary>\n/// One.\n///\n/// Two.\n/// </summary>\n"
        );
    }

    #[test]
    fn doc_comment_indented() {
        assert_eq!(
            code(CommentType::Doc, 1, &[" One."]),
            "    /// <summary>\n    /// One.\n    /// </summary>\n"
        );
    }
}
