use crate::c::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with modifiers.
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

    /// Writes the modifiers in their conventional order.
    ///
    /// # Note
    /// - A space is written after each.
    fn write_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            modifier.write(b);
            b.space();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement;
    use crate::c::SignatureDec;

    #[test]
    fn no_modifiers() {
        let dec: SignatureDec = SignatureDec::from("foo");
        assert!(dec.modifiers().is_empty());
        assert_eq!(dec.to_code(), "void foo(void);\n");
    }

    #[test]
    fn modifiers_in_the_conventional_order() {
        let dec: SignatureDec = SignatureDec::from("foo")
            .with_modifier(Modifier::Inline)
            .with_modifier(Modifier::Static);
        assert_eq!(dec.modifiers().len(), 2);
        assert_eq!(dec.to_code(), "static inline void foo(void);\n");
    }

    #[test]
    fn each_modifier_once() {
        let dec: SignatureDec = SignatureDec::from("foo")
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Static);
        assert_eq!(dec.modifiers().len(), 1);
        assert_eq!(dec.to_code(), "static void foo(void);\n");
    }
}
