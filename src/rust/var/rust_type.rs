use crate::rust::RustType::*;
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
    pub fn to_slice(self) -> Self {
        Slice(Box::new(self))
    }
}

impl RustType {
    //! Generics Types

    /// Adds the generic type.
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
    pub fn to_option(self) -> Self {
        Self::from("Option").with_generic(self)
    }

    /// Converts the type to a `Vec` of itself.
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
                b.write("(");
                if let Some((first, rest)) = members.split_first() {
                    first.write(b);
                    for member in rest {
                        b.write(", ");
                        member.write(b);
                    }
                }
                b.write(")");
            }
            Slice(base) => {
                b.write("[");
                base.write(b);
                b.write("]");
            }
            Generic { base, generics } => {
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

impl Display for RustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut b: CodeBuffer = CodeBuffer::new("", "", 64);
        self.write(&mut b);
        write!(f, "{}", b)
    }
}
