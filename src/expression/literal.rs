use crate::expression::Expression;
use crate::CodeBuffer;

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

#[cfg(test)]
mod tests {
    use crate::{CodeBuffer, Literal};

    #[test]
    fn write() {
        let literal: Literal = "value".into();
        let result: String = CodeBuffer::display_expression(&literal);
        assert_eq!(result, "value");
    }
}
