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
