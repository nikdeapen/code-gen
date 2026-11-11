use crate::cpp::Generic;
use crate::{CodeBuffer, Expression};

/// An element with template parameters.
pub trait WithGenerics: Sized {
    /// Gets the template parameters.
    fn generics(&self) -> &[Generic];

    /// Adds the template parameter.
    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>;

    /// Adds the template parameter.
    #[must_use]
    fn with_generic<G>(mut self, generic: G) -> Self
    where
        G: Into<Generic>,
    {
        self.add_generic(generic);
        self
    }

    /// Writes the template line, if there are template parameters. (ex: `template <typename T>`)
    fn write_generics(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.generics().split_first() {
            b.indent(level);
            b.write("template <");
            first.write(b);
            for generic in rest {
                b.write(", ");
                generic.write(b);
            }
            b.push('>');
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::{SignatureDec, Var};

    #[test]
    fn write_generics() {
        let dec = SignatureDec::from("foo")
            .with_generic("T")
            .with_generic(Var::from(("N", "std::size_t")));
        let mut b = CodeBuffer::default();
        dec.write_generics(&mut b, 1);
        assert_eq!(b.peek(), "    template <typename T, std::size_t N>\n");
    }

    #[test]
    fn write_no_generics() {
        let dec = SignatureDec::from("foo");
        let mut b = CodeBuffer::default();
        dec.write_generics(&mut b, 1);
        assert_eq!(b.peek(), "");
    }
}
