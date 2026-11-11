use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A Kotlin type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum KotlinType {
    /// A named type. (ex: `Int` or `java.util.UUID`)
    Named(String),

    /// A generic type. (ex: `List<Int>`)
    Generic {
        base: Box<KotlinType>,
        generics: Vec<KotlinType>,
    },

    /// A nullable type. (ex: `Int?`)
    Nullable(Box<KotlinType>),
}

impl<S: Into<String>> From<S> for KotlinType {
    fn from(name: S) -> Self {
        Self::Named(name.into())
    }
}

impl KotlinType {
    //! Generic Types

    /// Adds the type argument.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<KotlinType>,
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

impl KotlinType {
    //! Nullable Types

    /// Converts the type to a nullable type of itself.
    ///
    /// # Note
    /// - A nullable type stays as it is.
    #[must_use]
    pub fn to_nullable(self) -> Self {
        match self {
            Self::Nullable(_) => self,
            base => Self::Nullable(Box::new(base)),
        }
    }
}

impl Expression for KotlinType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Named(name) => b.write(name.as_str()),
            Self::Generic { base, generics } => {
                base.write(b);
                if let Some((first, rest)) = generics.split_first() {
                    b.push('<');
                    first.write(b);
                    for generic in rest {
                        b.write(", ");
                        generic.write(b);
                    }
                    b.push('>');
                }
            }
            Self::Nullable(base) => {
                base.write(b);
                b.push('?');
            }
        }
    }
}

impl Display for KotlinType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        assert_eq!(KotlinType::from("Int").to_code(), "Int");
        assert_eq!(
            KotlinType::from(String::from("java.util.UUID")).to_code(),
            "java.util.UUID"
        );
    }

    #[test]
    fn generic_type() {
        let t = KotlinType::from("List").with_generic("Int");
        assert_eq!(t.to_code(), "List<Int>");
    }

    #[test]
    fn generic_type_multiple() {
        let t = KotlinType::from("Map")
            .with_generic("String")
            .with_generic(KotlinType::from("List").with_generic("UInt"));
        assert_eq!(t.to_code(), "Map<String, List<UInt>>");
    }

    #[test]
    fn generic_type_without_generics() {
        let t = KotlinType::Generic {
            base: Box::new(KotlinType::from("List")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "List");
    }

    #[test]
    fn nullable_type() {
        let t = KotlinType::from("String").to_nullable();
        assert_eq!(t.to_code(), "String?");
    }

    #[test]
    fn nullable_generic_type() {
        let t = KotlinType::from("List")
            .with_generic(KotlinType::from("Int").to_nullable())
            .to_nullable();
        assert_eq!(t.to_code(), "List<Int?>?");
    }

    #[test]
    fn nullable_type_stays_nullable() {
        let t = KotlinType::from("String").to_nullable().to_nullable();
        assert_eq!(t.to_code(), "String?");
    }

    #[test]
    fn display_matches_to_code() {
        let t = KotlinType::from("Map")
            .with_generic("String")
            .with_generic(KotlinType::from("Int").to_nullable());
        assert_eq!(format!("{t}"), "Map<String, Int?>");
    }
}
