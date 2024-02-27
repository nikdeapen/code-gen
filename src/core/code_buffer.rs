use crate::Statement;
use std::fmt::{Display, Formatter};

/// Responsible for buffering code.
#[derive(Clone, Debug)]
pub struct CodeBuffer {
    indent: String,
    line_ending: String,
    code: String,
}

impl CodeBuffer {
    //! Constants

    /// The default indent.
    pub const DEFAULT_INDENT: &'static str = "\t";

    /// The default line-ending.
    pub const DEFAULT_LINE_ENDING: &'static str = "\n";

    /// The default buffer capacity. (4 KiB)
    pub const DEFAULT_CAPACITY: usize = 4 * 1024;
}

impl CodeBuffer {
    //! Construction

    /// Creates a new code buffer.
    pub fn new<S0, S1>(indent: S0, line_ending: S1, capacity: usize) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
    {
        Self {
            indent: indent.into(),
            line_ending: line_ending.into(),
            code: String::with_capacity(capacity),
        }
    }
}

impl Default for CodeBuffer {
    fn default() -> Self {
        Self::new(
            Self::DEFAULT_INDENT,
            Self::DEFAULT_LINE_ENDING,
            Self::DEFAULT_CAPACITY,
        )
    }
}

impl CodeBuffer {
    //! Writing

    /// Writes the code.
    pub fn write(&mut self, code: &str) {
        self.code.push_str(code);
    }

    /// Writes the indent level.
    pub fn indent(&mut self, level: usize) {
        for _ in 0..level {
            self.code.push_str(self.indent.as_mut_str());
        }
    }

    /// Writes a line-ending.
    pub fn end_line(&mut self) {
        self.code.push_str(self.line_ending.as_str());
    }

    /// Writes a line of code at the indent level with a line-ending.
    pub fn line(&mut self, level: usize, code: &str) {
        self.indent(level);
        self.write(code);
        self.end_line();
    }

    /// Writes a single space.
    pub fn space(&mut self) {
        self.code.push_str(" ");
    }
}

impl CodeBuffer {
    //! Access

    /// Peeks at the buffered code.
    pub fn peek(&self) -> &str {
        self.code.as_str()
    }

    /// Exports the buffered code.
    pub fn export(self) -> String {
        self.code
    }

    /// Clears the buffered code.
    pub fn clear(&mut self) {
        self.code.clear();
    }
}

impl CodeBuffer {
    //! Display

    /// Converts the statement to a string using the default code buffer.
    pub fn display_statement<S>(statement: &S) -> String
    where
        S: Statement,
    {
        let mut b: CodeBuffer = CodeBuffer::default();
        statement.write(&mut b, 0);
        b.export()
    }
}

impl Display for CodeBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.peek())
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeBuffer;

    #[test]
    fn write_peek() {
        let mut b: CodeBuffer = CodeBuffer::default();

        b.indent(1);
        b.write("one");
        b.end_line();
        b.line(2, "two");
        b.space();

        assert_eq!(b.peek(), "\tone\n\t\ttwo\n ");
    }

    #[test]
    fn export() {
        let mut b: CodeBuffer = CodeBuffer::default();

        b.write("code");

        assert_eq!(b.export(), "code");
    }

    #[test]
    fn clear() {
        let mut b: CodeBuffer = CodeBuffer::default();

        b.write("code");
        b.clear();

        assert_eq!(b.export(), "");
    }

    #[test]
    fn display() {
        let mut b: CodeBuffer = CodeBuffer::default();

        b.write("code");

        assert_eq!(b.to_string(), "code");
    }
}
