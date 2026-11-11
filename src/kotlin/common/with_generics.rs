use crate::kotlin::Generic;
use crate::{CodeBuffer, Expression};

/// An element with type parameters.
pub trait WithGenerics: Sized {
    /// Gets the type parameters.
    fn generics(&self) -> &[Generic];

    /// Adds the type parameter.
    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>;

    /// Adds the type parameter.
    #[must_use]
    fn with_generic<G>(mut self, generic: G) -> Self
    where
        G: Into<Generic>,
    {
        self.add_generic(generic);
        self
    }

    /// Writes the type parameters in brackets. (ex: `<T, U : Any>`)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::{Signature, Var};

    /// Writes the generic brackets of the `signature`.
    fn code(signature: &Signature) -> String {
        let mut b = CodeBuffer::default();
        signature.write_generic_brackets(&mut b);
        b.into()
    }

    #[test]
    fn no_generics() {
        assert_eq!(code(&Signature::from("f")), "");
    }

    #[test]
    fn one_generic() {
        assert_eq!(code(&Signature::from("f").with_generic("T")), "<T>");
    }

    #[test]
    fn many_generics() {
        let s = Signature::from("f")
            .with_generic("T")
            .with_generic(Var::from(("U", "Any")));
        assert_eq!(code(&s), "<T, U : Any>");
    }
}
