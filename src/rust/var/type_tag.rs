use crate::{CodeBuffer, Expression, WithName};
use crate::rust::PrimitiveType;
use crate::rust::TypeTag::*;

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// A primitive type.
    Primitive(PrimitiveType),

    /// A named type.
    Named(String),
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

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Named(name) => b.write(name.as_str()),
        }
    }
}
