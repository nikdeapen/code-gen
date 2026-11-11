use crate::java::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with keyword modifiers.
pub trait WithModifiers: Sized {
    /// Gets the modifiers.
    fn modifiers(&self) -> &BTreeSet<Modifier>;

    /// Adds a modifier.
    fn add_modifier(&mut self, modifier: Modifier);

    /// Adds a modifier.
    #[must_use]
    fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier(modifier);
        self
    }

    /// Writes the modifiers in their conventional order.
    fn write_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            modifier.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::Field;

    #[test]
    fn modifiers_are_written_in_order_and_once() {
        let field: Field = Field::from(("MAX", "int"))
            .with_modifier(Modifier::Final)
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Final);
        let mut b: CodeBuffer = CodeBuffer::default();
        field.write_modifiers(&mut b);
        assert_eq!(b.peek(), "static final ");
    }

    #[test]
    fn no_modifiers() {
        let field: Field = Field::from(("max", "int"));
        let mut b: CodeBuffer = CodeBuffer::default();
        field.write_modifiers(&mut b);
        assert_eq!(b.peek(), "");
    }
}
