use crate::rust::Var;
use crate::{CodeBuffer, Expression, WithName};

/// An element with function generics.
pub trait WithFnGenerics: Sized {
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

    /// Writes the generic brackets.
    fn write_generic_brackets(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.generics().split_first() {
            b.write("<");
            first.write_name(b);
            for generic in rest {
                b.write(", ");
                generic.write_name(b);
            }
            b.write(">");
        }
    }

    //// Writes the generic where clause.
    fn write_generic_where(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.generics().split_first() {
            b.write(" where ");
            first.write(b);
            for generic in rest {
                b.write(", ");
                generic.write(b);
            }
        }
    }
}
