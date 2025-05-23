use crate::go::GoPrimitive;
use crate::go::GoType::{Named, Primitive};
use crate::{CodeBuffer, Expression, WithName};
use std::fmt::{Display, Formatter};

/// A Go type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum GoType {
    /// A primitive type.
    Primitive(GoPrimitive),

    /// A named type.
    Named(String),
}

impl From<GoPrimitive> for GoType {
    fn from(primitive: GoPrimitive) -> Self {
        Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for GoType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl Expression for GoType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.name()),
            Named(name) => b.write(name.as_str()),
        }
    }
}

impl Display for GoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut b: CodeBuffer = CodeBuffer::new("", "", 64);
        self.write(&mut b);
        write!(f, "{}", b)
    }
}
