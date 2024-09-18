use crate::rust::TypeTag::*;
use crate::rust::{PrimitiveType, Reference};
use crate::{CodeBuffer, Expression, WithName};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A reference type.
    Ref {
        reference: Reference,
        base: Box<TypeTag>,
    },

    /// A slice type.
    Slice(Box<TypeTag>),

    /// A named type.
    Named(String),

    /// The `Self` type.
    SelfType,
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
        Ref {
            reference: reference.into(),
            base: Box::new(self),
        }
    }

    /// Converts the type to a shared reference type.
    pub fn to_shared_ref(self) -> Self {
        self.to_ref_type(Reference::default())
    }

    /// Converts the type to a mutable reference type.
    pub fn to_mut_ref(self) -> Self {
        self.to_ref_type(Reference::default().with_mut())
    }
}

impl TypeTag {
    //! Slice Types

    /// Converts the type to a slice of itself.
    pub fn to_slice(self) -> Self {
        Slice(Box::new(self))
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Ref { reference, base } => {
                reference.write(b);
                base.write(b);
            }
            Slice(base) => {
                b.write("[");
                base.write(b);
                b.write("]");
            }
            Named(name) => b.write(name.as_str()),
            SelfType => b.write("Self"),
        }
    }
}
