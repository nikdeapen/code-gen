use crate::java::JavaPrimitive;
use crate::java::JavaType::{Array, Generic, Named, Primitive};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A Java type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum JavaType {
    /// A primitive type.
    Primitive(JavaPrimitive),

    /// A named type.
    Named(String),

    /// A generic type. A primitive type argument is written as its boxed type.
    Generic {
        base: Box<JavaType>,
        generics: Vec<JavaType>,
    },

    /// An array type.
    Array(Box<JavaType>),
}

impl From<JavaPrimitive> for JavaType {
    fn from(primitive: JavaPrimitive) -> Self {
        Primitive(primitive)
    }
}

impl<S: Into<String>> From<S> for JavaType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl JavaType {
    //! Generic Types

    /// Adds the generic type argument.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<JavaType>,
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

    /// Writes the type as a type argument.
    fn write_generic(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => b.write(primitive.boxed_name()),
            generic => generic.write(b),
        }
    }
}

impl JavaType {
    //! Array Types

    /// Converts the type to an array type of itself.
    #[must_use]
    pub fn to_array(self) -> Self {
        Array(Box::new(self))
    }
}

impl JavaType {
    //! Boxed Types

    /// Converts a primitive type to its boxed type.
    ///
    /// # Note
    /// - A boxed type can be null. Use it for an optional value.
    /// - Any other type is kept as it is.
    #[must_use]
    pub fn to_boxed(self) -> Self {
        match self {
            Primitive(primitive) => Named(primitive.boxed_name().to_string()),
            boxed => boxed,
        }
    }
}

impl Expression for JavaType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Primitive(primitive) => primitive.write(b),
            Named(name) => b.write(name.as_str()),
            Generic { base, generics } => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.push('<');
                    first.write_generic(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write_generic(b);
                    }
                    b.push('>');
                }
            }
            Array(base) => {
                base.write(b);
                b.write("[]");
            }
        }
    }
}

impl Display for JavaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitive_type() {
        let t: JavaType = JavaType::from(JavaPrimitive::Int);
        assert_eq!(t.to_code(), "int");
    }

    #[test]
    fn named_type() {
        let t: JavaType = JavaType::from("java.util.UUID");
        assert_eq!(t.to_code(), "java.util.UUID");
    }

    #[test]
    fn generic_type() {
        let t: JavaType = JavaType::from("List").with_generic("String");
        assert_eq!(t.to_code(), "List<String>");
    }

    #[test]
    fn generic_type_multiple() {
        let t: JavaType = JavaType::from("Map")
            .with_generic("String")
            .with_generic(JavaType::from("List").with_generic("Point"));
        assert_eq!(t.to_code(), "Map<String, List<Point>>");
    }

    #[test]
    fn generic_type_boxes_primitives() {
        let t: JavaType = JavaType::from("Map")
            .with_generic(JavaPrimitive::Int)
            .with_generic(JavaType::from(JavaPrimitive::Byte).to_array());
        assert_eq!(t.to_code(), "Map<Integer, byte[]>");
    }

    #[test]
    fn generic_type_without_arguments() {
        let t: JavaType = Generic {
            base: Box::new(JavaType::from("List")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "List");
    }

    #[test]
    fn array_type() {
        let t: JavaType = JavaType::from(JavaPrimitive::Byte).to_array();
        assert_eq!(t.to_code(), "byte[]");
    }

    #[test]
    fn array_of_arrays() {
        let t: JavaType = JavaType::from("String").to_array().to_array();
        assert_eq!(t.to_code(), "String[][]");
    }

    #[test]
    fn boxed_primitive() {
        let t: JavaType = JavaType::from(JavaPrimitive::Long).to_boxed();
        assert_eq!(t.to_code(), "Long");
    }

    #[test]
    fn boxed_keeps_other_types() {
        let t: JavaType = JavaType::from("String").to_boxed();
        assert_eq!(t.to_code(), "String");
    }

    #[test]
    fn display_matches_to_code() {
        let t: JavaType = JavaType::from("List").with_generic(JavaPrimitive::Int);
        assert_eq!(format!("{t}"), "List<Integer>");
    }
}
