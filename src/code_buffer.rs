use std::fmt::{Display, Formatter};

use crate::{Expression, Statement};

/// Responsible for buffering code.
#[derive(Clone, Debug)]
pub struct CodeBuffer {
    indent: String,
    line_ending: String,
    code: String,
}

impl CodeBuffer {
    //! Constructors

    /// Creates a new code buffer.
    pub fn new(indent: &str, line_ending: &str, capacity: usize) -> Self {
        Self {
            indent: indent.to_string(),
            line_ending: line_ending.to_string(),
            code: String::with_capacity(capacity),
        }
    }
}

impl Default for CodeBuffer {
    fn default() -> Self {
        Self::new("\t", "\n", 4 * 1024)
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
            self.code.push_str(self.indent.as_str());
        }
    }

    /// Writes a line-ending.
    pub fn end_line(&mut self) {
        self.code.push_str(self.line_ending.as_str());
    }

    /// Writes a line with the indent level and a line ending.
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
    //! Exporting

    /// Peeks at the buffered code.
    pub fn peek(&self) -> &str {
        self.code.as_str()
    }

    /// Exports the buffered code.
    pub fn export(self) -> String {
        self.code
    }
}

impl CodeBuffer {
    //! Clearing

    /// Clears the buffered code.
    pub fn clear(&mut self) {
        self.code.clear()
    }
}

impl CodeBuffer {
    //! Display

    /// Converts the expression to a string.
    pub fn display_expression<E: Expression>(expression: &E) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        expression.write(&mut b);
        b.export()
    }

    /// Converts the statement to a string.
    pub fn display_statement<S: Statement>(statement: &S) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        statement.write(&mut b, 0);
        b.export()
    }
}

impl Display for CodeBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code.as_str())
    }
}
