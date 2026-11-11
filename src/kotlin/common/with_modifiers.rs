use crate::kotlin::Modifier;
use crate::{CodeBuffer, Expression};
use std::collections::BTreeSet;

/// An element with keyword modifiers.
pub trait WithModifiers: Sized {
    /// Gets the modifiers, in the order they are written in.
    fn modifiers(&self) -> &BTreeSet<Modifier>;

    /// Adds the modifier.
    fn add_modifier(&mut self, modifier: Modifier);

    /// Adds the modifier.
    #[must_use]
    fn with_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier(modifier);
        self
    }

    /// Writes the modifiers, each followed by a space.
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
    use crate::kotlin::Function;

    /// Writes the modifiers of the `function`.
    fn code(function: &Function) -> String {
        let mut b = CodeBuffer::default();
        function.write_modifiers(&mut b);
        b.into()
    }

    #[test]
    fn no_modifiers() {
        assert_eq!(code(&Function::from("f")), "");
    }

    #[test]
    fn modifiers_in_the_conventional_order() {
        let f = Function::from("f")
            .with_modifier(Modifier::Data)
            .with_modifier(Modifier::Value)
            .with_modifier(Modifier::Suspend)
            .with_modifier(Modifier::Override)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Sealed);
        assert_eq!(code(&f), "sealed const override suspend value data ");
    }

    #[test]
    fn modifier_added_twice() {
        let f = Function::from("f")
            .with_modifier(Modifier::Suspend)
            .with_modifier(Modifier::Override)
            .with_modifier(Modifier::Suspend);
        assert_eq!(code(&f), "override suspend ");
    }
}
