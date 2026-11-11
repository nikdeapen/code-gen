use crate::cpp::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with keyword modifiers.
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

    /// Writes the modifiers that go before the declaration, each followed by a space.
    fn write_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            if !modifier.is_trailing() {
                modifier.write(b);
                b.space();
            }
        }
    }

    /// Writes the modifiers that go after the parameters of a function, each after a space.
    fn write_trailing_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            if modifier.is_trailing() {
                b.space();
                modifier.write(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::Signature;

    #[test]
    fn modifiers_are_ordered_and_unique() {
        let signature = Signature::from("foo")
            .with_modifier(Modifier::Noexcept)
            .with_modifier(Modifier::Constexpr)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Constexpr);
        let modifiers: Vec<Modifier> = signature.modifiers().iter().copied().collect();
        assert_eq!(
            modifiers,
            [
                Modifier::Static,
                Modifier::Constexpr,
                Modifier::Const,
                Modifier::Noexcept
            ]
        );
    }

    #[test]
    fn write_modifiers() {
        let signature = Signature::from("foo")
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Constexpr)
            .with_modifier(Modifier::Static);
        let mut b = CodeBuffer::default();
        signature.write_modifiers(&mut b);
        assert_eq!(b.peek(), "static constexpr ");
    }

    #[test]
    fn write_trailing_modifiers() {
        let signature = Signature::from("foo")
            .with_modifier(Modifier::Override)
            .with_modifier(Modifier::Virtual)
            .with_modifier(Modifier::Noexcept)
            .with_modifier(Modifier::Const);
        let mut b = CodeBuffer::default();
        signature.write_trailing_modifiers(&mut b);
        assert_eq!(b.peek(), " const noexcept override");
    }

    #[test]
    fn write_no_modifiers() {
        let signature = Signature::from("foo");
        let mut b = CodeBuffer::default();
        signature.write_modifiers(&mut b);
        signature.write_trailing_modifiers(&mut b);
        assert_eq!(b.peek(), "");
    }
}
