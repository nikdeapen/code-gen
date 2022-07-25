use crate::{CodeBuffer, Expression};

/// A literal expression.
#[derive(Clone, Debug)]
pub struct Literal {
    value: String,
}

impl From<String> for Literal {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl Expression for Literal {
    fn write(&self, b: &mut CodeBuffer) {
        b.write(self.value.as_str());
    }
}
