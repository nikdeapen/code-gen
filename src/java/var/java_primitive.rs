use crate::{CodeBuffer, Expression, WithName};

/// A Java primitive.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum JavaPrimitive {
    Boolean,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
}

impl WithName for JavaPrimitive {
    fn name(&self) -> &str {
        match self {
            Self::Boolean => "boolean",
            Self::Byte => "byte",
            Self::Short => "short",
            Self::Int => "int",
            Self::Long => "long",
            Self::Float => "float",
            Self::Double => "double",
            Self::Char => "char",
        }
    }
}

impl JavaPrimitive {
    //! Properties

    /// Gets the name of the boxed type. (ex: `Integer`)
    #[must_use]
    pub const fn boxed_name(&self) -> &str {
        match self {
            Self::Boolean => "Boolean",
            Self::Byte => "Byte",
            Self::Short => "Short",
            Self::Int => "Integer",
            Self::Long => "Long",
            Self::Float => "Float",
            Self::Double => "Double",
            Self::Char => "Character",
        }
    }
}

impl Expression for JavaPrimitive {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every primitive with its name & its boxed name.
    const PRIMITIVES: [(JavaPrimitive, &str, &str); 8] = [
        (JavaPrimitive::Boolean, "boolean", "Boolean"),
        (JavaPrimitive::Byte, "byte", "Byte"),
        (JavaPrimitive::Short, "short", "Short"),
        (JavaPrimitive::Int, "int", "Integer"),
        (JavaPrimitive::Long, "long", "Long"),
        (JavaPrimitive::Float, "float", "Float"),
        (JavaPrimitive::Double, "double", "Double"),
        (JavaPrimitive::Char, "char", "Character"),
    ];

    #[test]
    fn names() {
        for (primitive, name, _) in PRIMITIVES {
            assert_eq!(primitive.to_code(), name);
        }
    }

    #[test]
    fn boxed_names() {
        for (primitive, _, boxed_name) in PRIMITIVES {
            assert_eq!(primitive.boxed_name(), boxed_name);
        }
    }
}
