use crate::csharp::CSharpPrimitive;
use crate::csharp::CSharpType::{Array, Generic, Named, Nullable, Primitive};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A C# type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CSharpType {
    Primitive(CSharpPrimitive),

    /// A type by its name, which can be qualified. (ex: `System.Guid`)
    Named(String),

    /// A type with type arguments. (ex: `List<int>`)
    Generic {
        base: Box<CSharpType>,
        generics: Vec<CSharpType>,
    },

    /// An array type. (ex: `byte[]`)
    Array(Box<CSharpType>),

    /// A nullable type. (ex: `string?`)
    Nullable(Box<CSharpType>),
}

impl From<CSharpPrimitive> for CSharpType {
    fn from(primitive: CSharpPrimitive) -> Self {
        Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for CSharpType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl CSharpType {
    //! Generic Types

    /// Adds the type argument.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<CSharpType>,
    {
        match self {
            Generic { base, mut generics } => {
                generics.push(generic.into());
                Generic { base, generics }
            }
            base => Generic {
                base: Box::new(base),
                generics: vec![generic.into()],
            },
        }
    }
}

impl CSharpType {
    //! Array Types

    /// Converts the type to an array type of itself.
    #[must_use]
    pub fn to_array(self) -> Self {
        Array(Box::new(self))
    }
}

impl CSharpType {
    //! Nullable Types

    /// Converts the type to a nullable type of itself.
    ///
    /// # Note
    /// - A nullable type stays as it is, since C# has no `T??`.
    #[must_use]
    pub fn to_nullable(self) -> Self {
        match self {
            Nullable(base) => Nullable(base),
            base => Nullable(Box::new(base)),
        }
    }
}

impl Expression for CSharpType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => primitive.write(b),
            Named(name) => b.write(name.as_str()),
            Generic { base, generics } => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.push('<');
                    first.write(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write(b);
                    }
                    b.push('>');
                }
            }
            Array(base) => {
                base.write(b);
                b.write("[]");
            }
            Nullable(base) => {
                base.write(b);
                b.push('?');
            }
        }
    }
}

impl Display for CSharpType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_type() {
        let t: CSharpType = CSharpType::from(CSharpPrimitive::UnsignedInt32);
        assert_eq!(t.to_code(), "uint");
    }

    #[test]
    fn named_type() {
        let t: CSharpType = CSharpType::from("System.Guid");
        assert_eq!(t.to_code(), "System.Guid");
    }

    #[test]
    fn generic_type() {
        let t: CSharpType = CSharpType::from("List").with_generic(CSharpPrimitive::String);
        assert_eq!(t.to_code(), "List<string>");
    }

    #[test]
    fn generic_type_multiple() {
        let t: CSharpType = CSharpType::from("Dictionary")
            .with_generic(CSharpPrimitive::String)
            .with_generic("Point");
        assert_eq!(t.to_code(), "Dictionary<string, Point>");
    }

    #[test]
    fn generic_type_without_arguments() {
        let t: CSharpType = Generic {
            base: Box::new(CSharpType::from("List")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "List");
    }

    #[test]
    fn array_type() {
        let t: CSharpType = CSharpType::from(CSharpPrimitive::UnsignedInt8).to_array();
        assert_eq!(t.to_code(), "byte[]");
    }

    #[test]
    fn nullable_type() {
        let t: CSharpType = CSharpType::from(CSharpPrimitive::String).to_nullable();
        assert_eq!(t.to_code(), "string?");
    }

    #[test]
    fn nullable_type_stays_as_it_is() {
        let t: CSharpType = CSharpType::from("Point").to_nullable().to_nullable();
        assert_eq!(t.to_code(), "Point?");
    }

    #[test]
    fn nested_types() {
        let t: CSharpType = CSharpType::from("List")
            .with_generic(CSharpType::from(CSharpPrimitive::SignedInt32).to_nullable())
            .to_array()
            .to_nullable();
        assert_eq!(t.to_code(), "List<int?>[]?");
    }

    #[test]
    fn display_matches_to_code() {
        let t: CSharpType = CSharpType::from("List").with_generic("Point");
        assert_eq!(format!("{t}"), t.to_code());
    }
}
