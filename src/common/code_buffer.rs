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

    /// The default indent. (4 spaces)
    pub const DEFAULT_INDENT: &'static str = "    ";

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

    /// Creates a new code buffer with the default indent and line-ending and the given `capacity`.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::new(Self::DEFAULT_INDENT, Self::DEFAULT_LINE_ENDING, capacity)
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

impl From<CodeBuffer> for String {
    fn from(buffer: CodeBuffer) -> Self {
        buffer.code
    }
}

impl CodeBuffer {
    //! Access

    /// Gets the length of the buffered code in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.code.len()
    }

    /// Checks if the buffered code is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    /// Peeks at the buffered code.
    #[must_use]
    pub fn peek(&self) -> &str {
        self.code.as_str()
    }

    /// Clears the buffered code.
    pub fn clear(&mut self) {
        self.code.clear();
    }
}

impl CodeBuffer {
    //! Writing

    /// Writes the `code`.
    pub fn write(&mut self, code: &str) {
        self.code.push_str(code);
    }

    /// Writes the indent `level`.
    pub fn indent(&mut self, level: usize) {
        for _ in 0..level {
            self.code.push_str(self.indent.as_str());
        }
    }

    /// Writes a line-ending.
    pub fn end_line(&mut self) {
        self.code.push_str(self.line_ending.as_str());
    }

    /// Writes a line of `code` at the indent `level` with a line-ending.
    pub fn line(&mut self, level: usize, code: &str) {
        self.indent(level);
        self.write(code);
        self.end_line();
    }

    /// Writes a single character.
    pub fn push(&mut self, c: char) {
        self.code.push(c);
    }

    /// Writes a single space.
    pub fn space(&mut self) {
        self.code.push(' ');
    }
}

impl Display for CodeBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.peek())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_empty() {
        let b = CodeBuffer::default();
        assert!(b.is_empty());
        assert_eq!(b.len(), 0);
        assert_eq!(b.peek(), "");
    }

    #[test]
    fn write_and_push() {
        let mut b = CodeBuffer::default();
        b.write("hello");
        b.push('!');
        assert_eq!(b.peek(), "hello!");
        assert_eq!(b.len(), 6);
        assert!(!b.is_empty());
    }

    #[test]
    fn indent_and_line() {
        let mut b = CodeBuffer::default();
        b.line(2, "code");
        assert_eq!(b.peek(), "        code\n");
    }

    #[test]
    fn space_and_end_line() {
        let mut b = CodeBuffer::default();
        b.write("a");
        b.space();
        b.write("b");
        b.end_line();
        assert_eq!(b.peek(), "a b\n");
    }

    #[test]
    fn clear() {
        let mut b = CodeBuffer::default();
        b.write("stuff");
        b.clear();
        assert!(b.is_empty());
        assert_eq!(b.peek(), "");
    }

    #[test]
    fn into_string() {
        let mut b = CodeBuffer::default();
        b.write("code");
        let s: String = b.into();
        assert_eq!(s, "code");
    }

    #[test]
    fn with_capacity() {
        let b = CodeBuffer::with_capacity(128);
        assert!(b.is_empty());
    }

    #[test]
    fn custom_indent_and_line_ending() {
        let mut b = CodeBuffer::new("\t", "\r\n", 64);
        b.line(1, "code");
        assert_eq!(b.peek(), "\tcode\r\n");
    }

    #[test]
    fn display() {
        let mut b = CodeBuffer::default();
        b.write("hello");
        assert_eq!(format!("{}", b), "hello");
    }
}
