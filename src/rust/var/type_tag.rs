use crate::rust::PrimitiveType;
use crate::rust::TypeTag::*;
use crate::{CodeBuffer, Expression, ToStaticStr};

/// A type tag.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeTag {
    /// The `Self` type.
    SelfTyped,

    /// A primitive type.
    Primitive(PrimitiveType),

    /// A named type.
    Named(String),

    /// A generic type.
    Generic((Box<TypeTag>, Vec<TypeTag>)),
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
    //! Generics

    /// Adds the generic type.
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<Self>,
    {
        if let Generic((base, mut generics)) = self {
            generics.push(generic.into());
            Generic((base, generics))
        } else {
            Generic((Box::new(self), vec![generic.into()]))
        }
    }

    /// Converts the type to an `Option` of itself.
    pub fn to_option(self) -> Self {
        Named("Option".to_string()).with_generic(self)
    }

    /// Converts the type to an `Vec` of itself.
    pub fn to_vec(self) -> Self {
        Named("Vec".to_string()).with_generic(self)
    }
}

impl Expression for TypeTag {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            SelfTyped => b.write("Self"),
            Primitive(primitive) => b.write(primitive.to_static_str()),
            Named(name) => b.write(name.as_str()),
            Generic((base, generics)) => {
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

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::*;
    use crate::rust::TypeTag;
    use crate::CodeBuffer;

    #[test]
    fn write_primitive() {
        let tag: TypeTag = UnsignedInt8.into();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "u8");
    }

    #[test]
    fn write_named() {
        let tag: TypeTag = "MyType".into();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType");
    }

    #[test]
    fn write_generic() {
        let tag: TypeTag = "MyType".into();

        let tag: TypeTag = tag.with_generic("u8");
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8>");

        let tag: TypeTag = tag.with_generic("u16");
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8, u16>");

        let tag: TypeTag = tag.with_generic("u32");
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "MyType<u8, u16, u32>");
    }

    #[test]
    fn write_option_vec() {
        let tag: TypeTag = "MyType".into();

        let tag: TypeTag = tag.to_option();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "Option<MyType>");

        let tag: TypeTag = tag.to_vec();
        let result: String = CodeBuffer::display_expression(&tag);
        assert_eq!(result, "Vec<Option<MyType>>");
    }
}
