use crate::csharp::CSharpType;
use crate::{CodeBuffer, Expression, WithName};

/// A C# primitive, which is a type with a keyword.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CSharpPrimitive {
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    SignedInt8,
    SignedInt16,
    SignedInt32,
    SignedInt64,
    Float32,
    Float64,
    Decimal,
    Boolean,
    Character,
    String,
    Object,
}

impl CSharpPrimitive {
    //! C# Types

    /// Converts the C# primitive to a C# type.
    #[must_use]
    pub const fn to_csharp_type(&self) -> CSharpType {
        CSharpType::Primitive(*self)
    }
}

impl WithName for CSharpPrimitive {
    fn name(&self) -> &str {
        match self {
            Self::UnsignedInt8 => "byte",
            Self::UnsignedInt16 => "ushort",
            Self::UnsignedInt32 => "uint",
            Self::UnsignedInt64 => "ulong",
            Self::SignedInt8 => "sbyte",
            Self::SignedInt16 => "short",
            Self::SignedInt32 => "int",
            Self::SignedInt64 => "long",
            Self::Float32 => "float",
            Self::Float64 => "double",
            Self::Decimal => "decimal",
            Self::Boolean => "bool",
            Self::Character => "char",
            Self::String => "string",
            Self::Object => "object",
        }
    }
}

impl Expression for CSharpPrimitive {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords() {
        assert_eq!(CSharpPrimitive::UnsignedInt8.to_code(), "byte");
        assert_eq!(CSharpPrimitive::UnsignedInt16.to_code(), "ushort");
        assert_eq!(CSharpPrimitive::UnsignedInt32.to_code(), "uint");
        assert_eq!(CSharpPrimitive::UnsignedInt64.to_code(), "ulong");
        assert_eq!(CSharpPrimitive::SignedInt8.to_code(), "sbyte");
        assert_eq!(CSharpPrimitive::SignedInt16.to_code(), "short");
        assert_eq!(CSharpPrimitive::SignedInt32.to_code(), "int");
        assert_eq!(CSharpPrimitive::SignedInt64.to_code(), "long");
        assert_eq!(CSharpPrimitive::Float32.to_code(), "float");
        assert_eq!(CSharpPrimitive::Float64.to_code(), "double");
        assert_eq!(CSharpPrimitive::Decimal.to_code(), "decimal");
        assert_eq!(CSharpPrimitive::Boolean.to_code(), "bool");
        assert_eq!(CSharpPrimitive::Character.to_code(), "char");
        assert_eq!(CSharpPrimitive::String.to_code(), "string");
        assert_eq!(CSharpPrimitive::Object.to_code(), "object");
    }

    #[test]
    fn to_csharp_type() {
        assert_eq!(
            CSharpPrimitive::SignedInt32.to_csharp_type(),
            CSharpType::Primitive(CSharpPrimitive::SignedInt32)
        );
    }
}
