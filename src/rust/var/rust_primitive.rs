use crate::rust::RustType;
use crate::{CodeBuffer, Expression, WithName};

/// A Rust primitive.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum RustPrimitive {
    UnsignedInt8,
    UnsignedInt16,
    UnsignedInt32,
    UnsignedInt64,
    UnsignedInt128,
    UnsignedIntSize,
    SignedInt8,
    SignedInt16,
    SignedInt32,
    SignedInt64,
    SignedInt128,
    SignedIntSize,
    Float32,
    Float64,
    Boolean,
    Character,
}

impl WithName for RustPrimitive {
    fn name(&self) -> &str {
        match self {
            Self::UnsignedInt8 => "u8",
            Self::UnsignedInt16 => "u16",
            Self::UnsignedInt32 => "u32",
            Self::UnsignedInt64 => "u64",
            Self::UnsignedInt128 => "u128",
            Self::UnsignedIntSize => "usize",
            Self::SignedInt8 => "i8",
            Self::SignedInt16 => "i16",
            Self::SignedInt32 => "i32",
            Self::SignedInt64 => "i64",
            Self::SignedInt128 => "i128",
            Self::SignedIntSize => "isize",
            Self::Float32 => "f32",
            Self::Float64 => "f64",
            Self::Boolean => "bool",
            Self::Character => "char",
        }
    }
}

impl RustPrimitive {
    //! Rust Types

    /// Converts the Rust primitive to a Rust type.
    pub const fn to_type_tag(&self) -> RustType {
        RustType::Primitive(*self)
    }
}

impl Expression for RustPrimitive {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
    }
}
