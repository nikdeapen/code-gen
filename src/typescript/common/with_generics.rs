use crate::typescript::Generic;
use crate::{CodeBuffer, Expression};

/// An element with generic parameters.
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

    /// Writes the generic brackets. (ex: `<T, U extends Shape>`)
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
    use crate::typescript::Signature;

    #[test]
    fn no_generics() {
        assert_eq!(Signature::from("foo").to_code(), "foo()");
    }

    #[test]
    fn one_generic() {
        let s = Signature::from("foo").with_generic("T");
        assert_eq!(s.to_code(), "foo<T>()");
    }

    #[test]
    fn many_generics() {
        let s = Signature::from("foo")
            .with_generic("T")
            .with_generic(("U", "Shape"));
        assert_eq!(s.to_code(), "foo<T, U extends Shape>()");
    }
}
