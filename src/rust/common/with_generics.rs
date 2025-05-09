use crate::rust::Var;
use crate::{CodeBuffer, Expression};

/// An element with generic type parameters.
pub trait WithGenerics: Sized {
    /// Gets the generic type parameters.
    fn generics(&self) -> &[Var];

    /// Adds the `generic` type parameter.
    fn add_generic<V>(&mut self, generic: V)
    where
        V: Into<Var>;

    /// Adds the `generic` type parameter.
    fn with_generic<V>(mut self, generic: V) -> Self
    where
        V: Into<Var>,
    {
        self.add_generic(generic);
        self
    }

    /// Writes the generic brackets. (ex: `<A: TypeA, B: TypeB>`)
    fn write_generic_brackets(&self, b: &mut CodeBuffer) {
        b.write("<");
        if let Some((first, rest)) = self.generics().split_first() {
            first.write(b);
            for generic in rest {
                b.write(", ");
                generic.write(b);
            }
        }
        b.write(">");
    }
}
