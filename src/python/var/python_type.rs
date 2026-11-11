use crate::python::PythonType::{Generic, Named, Union};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A Python type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum PythonType {
    /// A named type. (ex: `int` or `uuid.UUID`)
    Named(String),

    /// A generic type. (ex: `dict[str, int]`)
    Generic {
        base: Box<PythonType>,
        generics: Vec<PythonType>,
    },

    /// A union type. (ex: `int | str`)
    Union(Vec<PythonType>),
}

impl<S: Into<String>> From<S> for PythonType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl PythonType {
    //! Generic Types

    /// Adds a generic type. (ex: `list` becomes `list[int]`)
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<PythonType>,
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
}

impl PythonType {
    //! Union Types

    /// Converts the type to an optional type of itself. (ex: `int` becomes `int | None`)
    #[must_use]
    pub fn to_optional(self) -> Self {
        Union(vec![self, Self::from("None")])
    }
}

impl Expression for PythonType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Named(name) => b.write(name.as_str()),
            Generic { base, generics } => {
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
            Union(members) => {
                if let Some((first, rest)) = members.split_first() {
                    first.write(b);
                    for member in rest {
                        b.write(" | ");
                        member.write(b);
                    }
                }
            }
        }
    }
}

impl Display for PythonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        let t = PythonType::from("uuid.UUID");
        assert_eq!(t.to_code(), "uuid.UUID");
    }

    #[test]
    fn generic_type() {
        let t = PythonType::from("list").with_generic("int");
        assert_eq!(t.to_code(), "list[int]");
    }

    #[test]
    fn generic_type_multiple() {
        let t = PythonType::from("dict")
            .with_generic("str")
            .with_generic(PythonType::from("list").with_generic("int"));
        assert_eq!(t.to_code(), "dict[str, list[int]]");
    }

    #[test]
    fn generic_type_without_generics() {
        let t = Generic {
            base: Box::new(PythonType::from("list")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "list");
    }

    #[test]
    fn union_type() {
        let t = Union(vec![PythonType::from("int"), PythonType::from("str")]);
        assert_eq!(t.to_code(), "int | str");
    }

    #[test]
    fn union_type_without_members() {
        let t = Union(Vec::default());
        assert_eq!(t.to_code(), "");
    }

    #[test]
    fn optional_type() {
        let t = PythonType::from("int").to_optional();
        assert_eq!(t.to_code(), "int | None");
    }

    #[test]
    fn optional_generic_type() {
        let t = PythonType::from("list").with_generic("int").to_optional();
        assert_eq!(t.to_code(), "list[int] | None");
    }

    #[test]
    fn optional_union_type() {
        let t = Union(vec![PythonType::from("int"), PythonType::from("str")]).to_optional();
        assert_eq!(t.to_code(), "int | str | None");
    }

    #[test]
    fn generic_type_of_optional_types() {
        let t = PythonType::from("list").with_generic(PythonType::from("int").to_optional());
        assert_eq!(t.to_code(), "list[int | None]");
    }

    #[test]
    fn display_matches_to_code() {
        let t = PythonType::from("dict")
            .with_generic("str")
            .with_generic(PythonType::from("int").to_optional());
        assert_eq!(format!("{t}"), t.to_code());
        assert_eq!(format!("{t}"), "dict[str, int | None]");
    }
}
