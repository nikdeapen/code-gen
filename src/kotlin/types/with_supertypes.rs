use crate::kotlin::KotlinType;
use crate::{CodeBuffer, Expression};

/// An element with supertypes.
pub trait WithSupertypes: Sized {
    /// Gets the supertypes.
    fn supertypes(&self) -> &[KotlinType];

    /// Adds the supertype.
    ///
    /// # Note
    /// - A superclass is named with its constructor call. (ex: `Shape()`)
    fn add_supertype<T>(&mut self, supertype: T)
    where
        T: Into<KotlinType>;

    /// Adds the supertype.
    ///
    /// # Note
    /// - A superclass is named with its constructor call. (ex: `Shape()`)
    #[must_use]
    fn with_supertype<T>(mut self, supertype: T) -> Self
    where
        T: Into<KotlinType>,
    {
        self.add_supertype(supertype);
        self
    }

    /// Writes the supertypes, after their colon. (ex: ` : Shape, Comparable<Shape>`)
    fn write_supertypes(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.supertypes().split_first() {
            b.write(" : ");
            first.write(b);
            for supertype in rest {
                b.write(", ");
                supertype.write(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kotlin::Class;

    /// Writes the supertypes of the `class`.
    fn code(class: &Class) -> String {
        let mut b = CodeBuffer::default();
        class.write_supertypes(&mut b);
        b.into()
    }

    #[test]
    fn no_supertypes() {
        assert_eq!(code(&Class::from("Circle")), "");
    }

    #[test]
    fn one_supertype() {
        assert_eq!(
            code(&Class::from("Circle").with_supertype("Shape()")),
            " : Shape()"
        );
    }

    #[test]
    fn many_supertypes() {
        let c = Class::from("Circle")
            .with_supertype("Shape")
            .with_supertype(KotlinType::from("Comparable").with_generic("Circle"));
        assert_eq!(code(&c), " : Shape, Comparable<Circle>");
    }
}
