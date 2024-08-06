use crate::rust::var::reference::Reference;
use crate::rust::PrimitiveType;
use crate::rust::TypeTag::*;
use crate::{CodeBuffer, Expression, WithName};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A named type.
    Named(String),

    /// A reference type. `(reference, base)`
    Ref(Reference, Box<TypeTag>),

    /// A generic type. `(base, generics)`
    Generic(Box<TypeTag>, Vec<TypeTag>),
}

impl From<PrimitiveType> for TypeTag {
    fn from(primitive: PrimitiveType) -> Self {
        Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for TypeTag {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl TypeTag {
    //! Reference Types

    /// Converts the type to a reference type.
    pub fn to_ref_type<R>(self, reference: R) -> Self
    where
        R: Into<Reference>,
    {
        Ref(reference.into(), Box::new(self))
    }
}

impl TypeTag {
    //! Generics Types

    /// Adds the generic type.
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<TypeTag>,
    {
        match self {
            Generic(base, mut generics) => {
                generics.push(generic.into());
                Generic(base, generics)
            }
            base => Generic(Box::new(base), vec![generic.into()]),
        }
    }

    /// Converts the type to an `Option` of itself.
    pub fn to_option(self) -> Self {
        Self::from("Option").with_generic(self)
    }

    /// Converts the type to a `Vec` of itself.
    pub fn to_vec(self) -> Self {
        Self::from("Vec").with_generic(self)
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Named(name) => b.write(name.as_str()),
            Ref(reference, base) => {
                reference.write(b);
                base.write(b);
            }
            Generic(base, generics) => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.write("<");
                    first.write(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write(b);
                    }
                    b.write(">");
                }
            }
        }
    }
}
