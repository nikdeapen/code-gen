use crate::CodeBuffer;

/// The quotes that start & end a docstring.
const QUOTES: &str = "\"\"\"";

/// An element with comment lines.
pub trait WithComments: Sized {
    /// Gets the comment lines.
    fn comments(&self) -> &[String];

    /// Adds a comment line.
    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>;

    /// Adds a comment line.
    #[must_use]
    fn with_comment<S>(mut self, comment: S) -> Self
    where
        S: Into<String>,
    {
        self.add_comment(comment);
        self
    }

    /// Writes the comment lines as a docstring.
    ///
    /// # Note
    /// - A single line is written with the quotes on the same line.
    /// - A single line that ends with a quote gets a space after it, since four quotes in a row
    ///   end the docstring one quote early.
    fn write_comments(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.comments().split_first() {
            b.indent(level);
            b.write(QUOTES);
            b.write(escape(first).as_str());
            if rest.is_empty() {
                if first.ends_with('"') {
                    b.space();
                }
            } else {
                for line in rest {
                    b.end_line();
                    if !line.is_empty() {
                        b.indent(level);
                        b.write(escape(line).as_str());
                    }
                }
                b.end_line();
                b.indent(level);
            }
            b.write(QUOTES);
            b.end_line();
        }
    }
}

/// Escapes a line of a docstring.
///
/// # Note
/// - A backslash would start an escape sequence.
/// - Three quotes would end the docstring.
fn escape(line: &str) -> String {
    line.replace('\\', "\\\\").replace(QUOTES, "\\\"\\\"\\\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::python::Field;

    /// Writes the `lines` as the docstring of a field at indent level 1.
    fn docstring(lines: &[&str]) -> String {
        let mut field: Field = Field::from("x");
        for line in lines {
            field.add_comment(*line);
        }
        let mut b: CodeBuffer = CodeBuffer::default();
        field.write_comments(&mut b, 1);
        b.into()
    }

    #[test]
    fn no_lines() {
        assert_eq!(docstring(&[]), "");
    }

    #[test]
    fn one_line() {
        assert_eq!(docstring(&["The x."]), "    \"\"\"The x.\"\"\"\n");
    }

    #[test]
    fn many_lines() {
        assert_eq!(
            docstring(&["The x.", "", "It is never negative."]),
            "    \"\"\"The x.\n\n    It is never negative.\n    \"\"\"\n"
        );
    }

    #[test]
    fn one_line_ending_with_a_quote() {
        assert_eq!(docstring(&["The \"x\""]), "    \"\"\"The \"x\" \"\"\"\n");
    }

    #[test]
    fn many_lines_ending_with_a_quote() {
        assert_eq!(
            docstring(&["The \"x\"", "The \"y\""]),
            "    \"\"\"The \"x\"\n    The \"y\"\n    \"\"\"\n"
        );
    }

    #[test]
    fn line_with_a_backslash() {
        assert_eq!(
            docstring(&["A \\d digit."]),
            "    \"\"\"A \\\\d digit.\"\"\"\n"
        );
    }

    #[test]
    fn line_with_three_quotes() {
        assert_eq!(
            docstring(&["The \"\"\" quotes."]),
            "    \"\"\"The \\\"\\\"\\\" quotes.\"\"\"\n"
        );
    }
}
