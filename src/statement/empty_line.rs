use crate::statement::Statement;
use crate::CodeBuffer;

/// An empty line of code. (indent & line-ending)
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct EmptyLine {
    _nothing: (),
}

impl Statement for EmptyLine {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.line(level, "");
    }
}

#[cfg(test)]
mod tests {
    use crate::{CodeBuffer, EmptyLine};

    #[test]
    fn write() {
        let empty_line: EmptyLine = EmptyLine::default();
        let result: String = CodeBuffer::display_statement(&empty_line);
        assert_eq!(result, "\n");
    }
}
