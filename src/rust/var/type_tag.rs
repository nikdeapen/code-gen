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

    /// A reference type.
    Ref(Reference, Box<TypeTag>),
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

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Named(name) => b.write(name.as_str()),
            Ref(reference, base) => {
                reference.write(b);
                base.write(b);
            }
        }
    }
}
