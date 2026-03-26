use crate::rust::RustType::{Generic, Named, Primitive, Ref, Slice, Tuple};
use crate::rust::{Reference, RustPrimitive};
use crate::{CodeBuffer, Expression, WithName};
use std::fmt::{Display, Formatter};

/// A Rust type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RustType {
    /// A primitive type.
    Primitive(RustPrimitive),

    /// A named type.
    Named(String),

    /// A reference type.
    Ref {
        reference: Reference,
        base: Box<RustType>,
    },

    /// A tuple type.
    Tuple(Vec<RustType>),

    /// A slice type.
    Slice(Box<RustType>),

    /// A generic type.
    Generic {
        base: Box<RustType>,
        generics: Vec<RustType>,
    },
}

impl From<RustPrimitive> for RustType {
    fn from(primitive: RustPrimitive) -> Self {
        Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for RustType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl RustType {
    //! Reference Types

    /// Converts the type to a reference type of itself.
    #[must_use]
    pub fn to_ref<R>(self, reference: R) -> Self
    where
        R: Into<Reference>,
    {
        Ref {
            reference: reference.into(),
            base: Box::new(self),
        }
    }
}

impl RustType {
    //! Slice Types

    /// Converts the type to a slice type of itself.
    #[must_use]
    pub fn to_slice(self) -> Self {
        Slice(Box::new(self))
    }
}

impl RustType {
    //! Generic Types

    /// Adds the generic type.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<RustType>,
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

    /// Converts the type to an `Option` of itself.
    #[must_use]
    pub fn to_option(self) -> Self {
        Self::from("Option").with_generic(self)
    }

    /// Converts the type to a `Vec` of itself.
    #[must_use]
    pub fn to_vec(self) -> Self {
        Self::from("Vec").with_generic(self)
    }
}

impl Expression for RustType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Named(name) => b.write(name.as_str()),
            Ref { reference, base } => {
                reference.write(b);
                base.write(b);
            }
            Tuple(members) => {
                b.push('(');
                if let Some((first, rest)) = members.split_first() {
                    first.write(b);
                    for member in rest {
                        b.write(", ");
                        member.write(b);
                    }
                }
                b.push(')');
            }
            Slice(base) => {
                b.push('[');
                base.write(b);
                b.push(']');
            }
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
        }
    }
}

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive(primitive) => f.write_str(primitive.name()),
            Named(name) => f.write_str(name.as_str()),
            Ref { reference, base } => {
                Display::fmt(reference, f)?;
                Display::fmt(base, f)
            }
            Tuple(members) => {
                f.write_str("(")?;
                if let Some((first, rest)) = members.split_first() {
                    Display::fmt(first, f)?;
                    for member in rest {
                        f.write_str(", ")?;
                        Display::fmt(member, f)?;
                    }
                }
                f.write_str(")")
            }
            Slice(base) => {
                f.write_str("[")?;
                Display::fmt(base, f)?;
                f.write_str("]")
            }
            Generic { base, generics } => {
                Display::fmt(base, f)?;
                if let Some((first, rest)) = generics.split_first() {
                    f.write_str("<")?;
                    Display::fmt(first, f)?;
                    for generic in rest {
                        f.write_str(", ")?;
                        Display::fmt(generic, f)?;
                    }
                    f.write_str(">")?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::RustPrimitive;

    #[test]
    fn named_type() {
        let t = RustType::from("MyStruct");
        assert_eq!(t.to_code(), "MyStruct");
    }

    #[test]
    fn primitive_type() {
        let t = RustType::from(RustPrimitive::UnsignedInt32);
        assert_eq!(t.to_code(), "u32");
    }

    #[test]
    fn reference_type() {
        let t = RustType::from("str").to_ref(Reference::default());
        assert_eq!(t.to_code(), "&str");
    }

    #[test]
    fn mut_reference_type() {
        let t = RustType::from("Self").to_ref(Reference::default().with_mut());
        assert_eq!(t.to_code(), "&mut Self");
    }

    #[test]
    fn static_lifetime_reference() {
        let t = RustType::from("str").to_ref(Reference::default().with_static_lifetime());
        assert_eq!(t.to_code(), "&'static str");
    }

    #[test]
    fn named_lifetime_reference() {
        let t = RustType::from("str").to_ref(Reference::default().with_lifetime('a').unwrap());
        assert_eq!(t.to_code(), "&'a str");
    }

    #[test]
    fn slice_type() {
        let t = RustType::from(RustPrimitive::UnsignedInt8).to_slice();
        assert_eq!(t.to_code(), "[u8]");
    }

    #[test]
    fn generic_type() {
        let t = RustType::from("Vec").with_generic(RustType::from("String"));
        assert_eq!(t.to_code(), "Vec<String>");
    }

    #[test]
    fn generic_type_multiple() {
        let t = RustType::from("HashMap")
            .with_generic(RustType::from("String"))
            .with_generic(RustType::from(RustPrimitive::UnsignedInt32));
        assert_eq!(t.to_code(), "HashMap<String, u32>");
    }

    #[test]
    fn option_type() {
        let t = RustType::from("String").to_option();
        assert_eq!(t.to_code(), "Option<String>");
    }

    #[test]
    fn vec_type() {
        let t = RustType::from(RustPrimitive::UnsignedInt8).to_vec();
        assert_eq!(t.to_code(), "Vec<u8>");
    }

    #[test]
    fn tuple_type() {
        let t = Tuple(vec![
            RustType::from("String"),
            RustType::from(RustPrimitive::UnsignedInt32),
        ]);
        assert_eq!(t.to_code(), "(String, u32)");
    }

    #[test]
    fn display_matches_to_code() {
        let t = RustType::from("Vec").with_generic(RustType::from("String"));
        assert_eq!(format!("{t}"), t.to_code());
    }
}
