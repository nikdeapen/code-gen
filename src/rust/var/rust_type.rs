use crate::rust::RustType::*;
use crate::rust::{Reference, RustPrimitive};
use crate::{CodeBuffer, Expression, WithName};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RustType {
    /// A primitive type.
    Primitive(RustPrimitive),

    /// A reference type.
    Ref {
        reference: Reference,
        base: Box<RustType>,
    },

    /// A slice type.
    Slice(Box<RustType>),

    /// A named type.
    Named(String),

    /// The `Self` type.
    SelfType,

    /// A generic type.
    Generic {
        base: Box<RustType>,
        generics: Vec<RustType>,
    },

    /// A tuple type.
    Tuple(Vec<RustType>),
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

impl RustType {
    //! Slice Types

    /// Converts the type to a slice of itself.
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

    /// Converts the type to a `Result` of itself with the given `error_type`.
    pub fn to_result<T>(self, error_type: T) -> Self
    where
        T: Into<Self>,
    {
        Self::from("Result")
            .with_generic(self)
            .with_generic(error_type)
    }

    /// Converts the type to a `Result` of itself with the `std::io::Error` error type.
    pub fn to_io_result(self) -> Self {
        self.to_result(Self::from("std::io::Error"))
    }
}
impl Expression for RustType {
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
        }
    }
}
