use crate::typescript::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with modifiers.
pub trait WithModifiers: Sized {
    /// Gets the modifiers.
    fn modifiers(&self) -> &BTreeSet<Modifier>;

    /// Adds the `modifier`.
    fn add_modifier(&mut self, modifier: Modifier);

    /// Adds the `modifier`.
    #[must_use]
    fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier(modifier);
        self
    }

    /// Writes the modifiers.
    ///
    /// # Note
    /// - They are written in the order TypeScript requires, each one once.
    fn write_modifiers(&self, b: &mut CodeBuffer) {
        for modifier in self.modifiers() {
            modifier.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement;
    use crate::typescript::Field;

    #[test]
    fn no_modifiers() {
        assert_eq!(Field::from("x").to_code(), "x;\n");
    }

    #[test]
    fn modifiers_in_order() {
        let f = Field::from("x")
            .with_modifier(Modifier::Readonly)
            .with_modifier(Modifier::Override)
            .with_modifier(Modifier::Static);
        assert_eq!(f.to_code(), "static override readonly x;\n");
    }

    #[test]
    fn modifier_added_twice() {
        let f = Field::from("x")
            .with_modifier(Modifier::Static)
            .with_modifier(Modifier::Static);
        assert_eq!(f.to_code(), "static x;\n");
        assert_eq!(f.modifiers().len(), 1);
    }
}
