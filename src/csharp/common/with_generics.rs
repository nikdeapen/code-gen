use crate::csharp::Generic;
use crate::{CodeBuffer, Expression, WithName};

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

    /// Writes the generic brackets. (ex: `<TKey, TValue>`)
    fn write_generic_brackets(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.generics().split_first() {
            b.push('<');
            first.write_name(b);
            for generic in rest {
                b.write(", ");
                generic.write_name(b);
            }
            b.push('>');
        }
    }

    /// Writes the `where` clause of each type parameter with constraints, with a space before it.
    fn write_generic_where(&self, b: &mut CodeBuffer) {
        for generic in self.generics() {
            if let Some((first, rest)) = generic.constraints().split_first() {
                b.write(" where ");
                generic.write_name(b);
                b.write(" : ");
                first.write(b);
                for constraint in rest {
                    b.write(", ");
                    constraint.write(b);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeBuffer;
    use crate::csharp::{Generic, Signature, WithGenerics};

    /// Gets the generic brackets & the `where` clauses of the `signature`.
    fn code(signature: &Signature) -> (String, String) {
        let mut brackets: CodeBuffer = CodeBuffer::default();
        signature.write_generic_brackets(&mut brackets);
        let mut clauses: CodeBuffer = CodeBuffer::default();
        signature.write_generic_where(&mut clauses);
        (brackets.into(), clauses.into())
    }

    #[test]
    fn no_generics() {
        let s: Signature = Signature::from("Run");
        assert_eq!(code(&s), (String::new(), String::new()));
    }

    #[test]
    fn generics_without_constraints() {
        let s: Signature = Signature::from("Run")
            .with_generic("TKey")
            .with_generic("TValue");
        assert_eq!(code(&s), ("<TKey, TValue>".to_string(), String::new()));
    }

    #[test]
    fn generics_with_constraints() {
        let s: Signature = Signature::from("Run")
            .with_generic(
                Generic::from("TKey")
                    .with_constraint("class")
                    .with_constraint("new()"),
            )
            .with_generic("TOther")
            .with_generic(Generic::from("TValue").with_constraint("IDisposable"));
        assert_eq!(
            code(&s),
            (
                "<TKey, TOther, TValue>".to_string(),
                " where TKey : class, new() where TValue : IDisposable".to_string()
            )
        );
    }
}
