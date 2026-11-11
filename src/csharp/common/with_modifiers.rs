use crate::csharp::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with modifiers.
///
/// # Note
/// - The set keeps each modifier once, in the order they are written in.
pub trait WithModifiers: Sized {
    /// Gets the modifiers.
    fn modifiers(&self) -> &BTreeSet<Modifier>;

    /// Adds the modifier.
    fn add_modifier(&mut self, modifier: Modifier);

    /// Adds the modifier.
    #[must_use]
    fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier(modifier);
        self
    }

    /// Writes the modifiers.
    fn write_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            modifier.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeBuffer;
    use crate::csharp::{Field, Modifier, WithModifiers};

    #[test]
    fn no_modifiers() {
        let mut b: CodeBuffer = CodeBuffer::default();
        Field::from(("x", "int")).write_modifiers(&mut b);
        assert_eq!(b.peek(), "");
    }

    #[test]
    fn modifiers_in_order_and_once() {
        let field: Field = Field::from(("x", "int"))
            .with_modifier(Modifier::Readonly)
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Readonly);
        let mut b: CodeBuffer = CodeBuffer::default();
        field.write_modifiers(&mut b);
        assert_eq!(b.peek(), "static readonly ");
    }
}
