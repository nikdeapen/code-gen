use crate::go::GoType;
use crate::{CodeBuffer, Expression, WithName};

/// A Go primitive.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum GoPrimitive {
    Boolean,
    String,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    UnsignedInt,
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    UnsignedIntPointer,
    Rune,
    Float32,
    Float64,
}

impl WithName for GoPrimitive {
    fn name(&self) -> &str {
        match self {
            Self::Boolean => "bool",
            Self::String => "string",
            Self::Int => "int",
            Self::Int8 => "int8",
            Self::Int16 => "int16",
            Self::Int32 => "int32",
            Self::Int64 => "int64",
            Self::UnsignedInt => "uint",
            Self::UnsignedInt8 => "uint8",
            Self::UnsignedInt16 => "uint16",
            Self::UnsignedInt32 => "uint32",
            Self::UnsignedInt64 => "uint64",
            Self::UnsignedIntPointer => "uintptr",
            Self::Rune => "rune",
            Self::Float32 => "float32",
            Self::Float64 => "float64",
        }
    }
}

impl GoPrimitive {
    //! Go Types

    /// Converts the Go primitive to a Go type.
    pub const fn to_type_tag(&self) -> GoType {
        GoType::Primitive(*self)
    }
}

impl Expression for GoPrimitive {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
    }
}
