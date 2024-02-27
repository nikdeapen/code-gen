use std::fmt::{Display, Formatter};

use crate::rust::TypeTag;
use crate::{ToStaticStr, WithName};

/// A primitive type.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PrimitiveType {
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    UnsignedInt128,
    UnsignedIntArch,
    SignedInt8,
    SignedInt16,
    SignedInt32,
    SignedInt64,
    SignedInt128,
    SignedIntArch,
    Float32,
    Float64,
    Boolean,
    Character,
}

impl WithName for PrimitiveType {
    fn name(&self) -> &str {
        self.to_static_str()
    }
}

impl PrimitiveType {
    //! Conversions

    /// Converts the primitive type to a type tag.
    pub fn to_type_tag(&self) -> TypeTag {
        TypeTag::Primitive(*self)
    }
}

impl ToStaticStr for PrimitiveType {
    fn to_static_str(&self) -> &'static str {
        match self {
            Self::UnsignedInt8 => "u8",
            Self::UnsignedInt16 => "u16",
            Self::UnsignedInt32 => "u32",
            Self::UnsignedInt64 => "u64",
            Self::UnsignedInt128 => "u128",
            Self::UnsignedIntArch => "usize",
            Self::SignedInt8 => "i8",
            Self::SignedInt16 => "i16",
            Self::SignedInt32 => "i32",
            Self::SignedInt64 => "i64",
            Self::SignedInt128 => "i128",
            Self::SignedIntArch => "isize",
            Self::Float32 => "f32",
            Self::Float64 => "f64",
            Self::Boolean => "bool",
            Self::Character => "char",
        }
    }
}

impl Display for PrimitiveType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::*;
    use crate::rust::TypeTag;

    #[test]
    fn to_type_tag() {
        let result: TypeTag = UnsignedInt8.to_type_tag();
        let expected: TypeTag = TypeTag::Primitive(UnsignedInt8);
        assert_eq!(result, expected);
    }

    #[test]
    fn display() {
        assert_eq!(UnsignedInt8.to_string(), "u8");
        assert_eq!(UnsignedInt16.to_string(), "u16");
        assert_eq!(UnsignedInt32.to_string(), "u32");
        assert_eq!(UnsignedInt64.to_string(), "u64");
        assert_eq!(UnsignedInt128.to_string(), "u128");
        assert_eq!(UnsignedIntArch.to_string(), "usize");
        assert_eq!(SignedInt8.to_string(), "i8");
        assert_eq!(SignedInt16.to_string(), "i16");
        assert_eq!(SignedInt32.to_string(), "i32");
        assert_eq!(SignedInt64.to_string(), "i64");
        assert_eq!(SignedInt128.to_string(), "i128");
        assert_eq!(SignedIntArch.to_string(), "isize");
        assert_eq!(Float32.to_string(), "f32");
        assert_eq!(Float64.to_string(), "f64");
        assert_eq!(Boolean.to_string(), "bool");
        assert_eq!(Character.to_string(), "char");
    }
}
