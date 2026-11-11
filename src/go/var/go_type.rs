use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A Go type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum GoType {
    /// A named type. (ex: `uint32` or `time.Time`)
    Named(String),

    /// A pointer type. (ex: `*Point`)
    Pointer(Box<GoType>),

    /// A slice type. (ex: `[]byte`)
    Slice(Box<GoType>),

    /// An array type. (ex: `[16]byte`)
    Array { len: usize, base: Box<GoType> },

    /// A map type. (ex: `map[string]uint32`)
    Map {
        key: Box<GoType>,
        value: Box<GoType>,
    },

    /// A generic type with its type arguments. (ex: `Pair[K, V]`)
    Generic {
        base: Box<GoType>,
        generics: Vec<GoType>,
    },
}

impl<S: Into<String>> From<S> for GoType {
    fn from(name: S) -> Self {
        Self::Named(name.into())
    }
}

impl GoType {
    //! Conversions

    /// Converts the type to a pointer to itself.
    #[must_use]
    pub fn to_pointer(self) -> Self {
        Self::Pointer(Box::new(self))
    }

    /// Converts the type to a slice of itself.
    #[must_use]
    pub fn to_slice(self) -> Self {
        Self::Slice(Box::new(self))
    }

    /// Converts the type to an array of itself.
    #[must_use]
    pub fn to_array(self, len: usize) -> Self {
        Self::Array {
            len,
            base: Box::new(self),
        }
    }

    /// Converts the type to a map with itself as the value.
    #[must_use]
    pub fn to_map<K>(self, key: K) -> Self
    where
        K: Into<GoType>,
    {
        Self::Map {
            key: Box::new(key.into()),
            value: Box::new(self),
        }
    }
}

impl GoType {
    //! Generic Types

    /// Adds the type argument.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<GoType>,
    {
        match self {
            Self::Generic { base, mut generics } => {
                generics.push(generic.into());
                Self::Generic { base, generics }
            }
            base => Self::Generic {
                base: Box::new(base),
                generics: vec![generic.into()],
            },
        }
    }
}

impl Expression for GoType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Named(name) => b.write(name.as_str()),
            Self::Pointer(base) => {
                b.push('*');
                base.write(b);
            }
            Self::Slice(base) => {
                b.write("[]");
                base.write(b);
            }
            Self::Array { len, base } => {
                b.push('[');
                b.write(len.to_string().as_str());
                b.push(']');
                base.write(b);
            }
            Self::Map { key, value } => {
                b.write("map[");
                key.write(b);
                b.push(']');
                value.write(b);
            }
            Self::Generic { base, generics } => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.push('[');
                    first.write(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write(b);
                    }
                    b.push(']');
                }
            }
        }
    }
}

impl Display for GoType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        let t = GoType::from("time.Time");
        assert_eq!(t.to_code(), "time.Time");
    }

    #[test]
    fn pointer_type() {
        let t = GoType::from("Point").to_pointer();
        assert_eq!(t.to_code(), "*Point");
    }

    #[test]
    fn slice_type() {
        let t = GoType::from("byte").to_slice();
        assert_eq!(t.to_code(), "[]byte");
    }

    #[test]
    fn array_type() {
        let t = GoType::from("byte").to_array(16);
        assert_eq!(t.to_code(), "[16]byte");
    }

    #[test]
    fn map_type() {
        let t = GoType::from("uint32").to_map("string");
        assert_eq!(t.to_code(), "map[string]uint32");
    }

    #[test]
    fn generic_type() {
        let t = GoType::from("Option").with_generic("string");
        assert_eq!(t.to_code(), "Option[string]");
    }

    #[test]
    fn generic_type_multiple() {
        let t = GoType::from("Pair")
            .with_generic("string")
            .with_generic(GoType::from("Point").to_pointer());
        assert_eq!(t.to_code(), "Pair[string, *Point]");
    }

    #[test]
    fn generic_type_without_type_arguments() {
        let t = GoType::Generic {
            base: Box::new(GoType::from("Point")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "Point");
    }

    #[test]
    fn nested_type() {
        let t = GoType::from("Point")
            .to_pointer()
            .to_slice()
            .to_array(2)
            .to_map("string");
        assert_eq!(t.to_code(), "map[string][2][]*Point");
    }

    #[test]
    fn display_matches_to_code() {
        let t = GoType::from("Option")
            .with_generic(GoType::from("byte").to_slice())
            .to_pointer();
        assert_eq!(format!("{t}"), "*Option[[]byte]");
        assert_eq!(format!("{t}"), t.to_code());
    }
}
