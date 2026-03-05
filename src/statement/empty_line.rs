use crate::{CodeBuffer, Statement};

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
    use super::*;

    #[test]
    fn empty_line_at_level_0() {
        assert_eq!(EmptyLine::default().to_code(), "\n");
    }

    #[test]
    fn empty_line_at_level_1() {
        let mut b = CodeBuffer::default();
        EmptyLine::default().write(&mut b, 1);
        assert_eq!(b.peek(), "    \n");
    }
}
