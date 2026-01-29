use crate::CodeBuffer;
use crate::Expression;

/// A literal expression.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Literal {
    value: String,
}

impl<S: Into<String>> From<S> for Literal {
    fn from(value: S) -> Self {
        Self {
            value: value.into(),
        }
    }
}

impl Expression for Literal {
    fn write(&self, b: &mut CodeBuffer) {
        b.write(self.value.as_str());
    }
}
