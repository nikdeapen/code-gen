use crate::rust::Generic;
use crate::{CodeBuffer, Expression};

/// An element with generic parameters. Writes `<Name: Bounds, ...>` in brackets.
pub trait WithGenerics: Sized {
    /// Gets the generic parameters.
    fn generics(&self) -> &[Generic];

    /// Adds the `generic` parameter.
    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>;

    /// Adds the `generic` parameter.
    #[must_use]
    fn with_generic<G>(mut self, generic: G) -> Self
    where
        G: Into<Generic>,
    {
        self.add_generic(generic);
        self
    }

    /// Writes the generic brackets. (ex: `<'a, T: Clone>`)
    fn write_generic_brackets(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.generics().split_first() {
            b.push('<');
            first.write(b);
            for generic in rest {
                b.write(", ");
                generic.write(b);
            }
            b.push('>');
        }
    }
}
