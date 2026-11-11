use crate::typescript::TypeScriptType::{Array, Generic, Named, Tuple, Union};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A TypeScript type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum TypeScriptType {
    /// A named type. (ex: `number`, `Uint8Array` or `"circle"`)
    Named(String),

    /// A generic type. (ex: `Map<string, number>`)
    Generic {
        base: Box<TypeScriptType>,
        generics: Vec<TypeScriptType>,
    },

    /// An array type. (ex: `string[]`)
    Array(Box<TypeScriptType>),

    /// A tuple type. (ex: `[string, number]`)
    Tuple(Vec<TypeScriptType>),

    /// A union type. (ex: `Circle | Square`)
    ///
    /// # Note
    /// - A union of no types is `never`.
    Union(Vec<TypeScriptType>),
}

impl<S: Into<String>> From<S> for TypeScriptType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl TypeScriptType {
    //! Generic Types

    /// Adds the generic type.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<TypeScriptType>,
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

impl TypeScriptType {
    //! Array Types

    /// Converts the type to an array type of itself.
    #[must_use]
    pub fn to_array(self) -> Self {
        Array(Box::new(self))
    }
}

impl TypeScriptType {
    //! Union Types

    /// Adds the union `member`, converting the type to a union type of itself if needed.
    #[must_use]
    pub fn with_member<T>(self, member: T) -> Self
    where
        T: Into<TypeScriptType>,
    {
        match self {
            Union(mut members) => {
                members.push(member.into());
                Union(members)
            }
            base => Union(vec![base, member.into()]),
        }
    }

    /// Converts the type to a union type of itself & `undefined`.
    #[must_use]
    pub fn to_optional(self) -> Self {
        self.with_member("undefined")
    }
}

impl Expression for TypeScriptType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Named(name) => b.write(name.as_str()),
            Generic { base, generics } => {
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
            Array(element) => {
                if matches!(**element, Union(_)) {
                    b.push('(');
                    element.write(b);
                    b.push(')');
                } else {
                    element.write(b);
                }
                b.write("[]");
            }
            Tuple(members) => {
                b.push('[');
                if let Some((first, rest)) = members.split_first() {
                    first.write(b);
                    for member in rest {
                        b.write(", ");
                        member.write(b);
                    }
                }
                b.push(']');
            }
            Union(members) => {
                if let Some((first, rest)) = members.split_first() {
                    first.write(b);
                    for member in rest {
                        b.write(" | ");
                        member.write(b);
                    }
                } else {
                    b.write("never");
                }
            }
        }
    }
}

impl Display for TypeScriptType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        assert_eq!(TypeScriptType::from("number").to_code(), "number");
        assert_eq!(TypeScriptType::from("\"circle\"").to_code(), "\"circle\"");
    }

    #[test]
    fn generic_type() {
        let t = TypeScriptType::from("Promise").with_generic("Point");
        assert_eq!(t.to_code(), "Promise<Point>");
    }

    #[test]
    fn generic_type_multiple() {
        let t = TypeScriptType::from("Map")
            .with_generic("string")
            .with_generic(TypeScriptType::from("number").to_array());
        assert_eq!(t.to_code(), "Map<string, number[]>");
    }

    #[test]
    fn generic_type_without_generics() {
        let t = Generic {
            base: Box::new(TypeScriptType::from("Point")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "Point");
    }

    #[test]
    fn array_type() {
        let t = TypeScriptType::from("string").to_array();
        assert_eq!(t.to_code(), "string[]");
        assert_eq!(t.to_array().to_code(), "string[][]");
    }

    #[test]
    fn array_type_of_a_union() {
        let t = TypeScriptType::from("string").to_optional().to_array();
        assert_eq!(t.to_code(), "(string | undefined)[]");
    }

    #[test]
    fn tuple_type() {
        let t = Tuple(vec![
            TypeScriptType::from("string"),
            TypeScriptType::from("number"),
        ]);
        assert_eq!(t.to_code(), "[string, number]");
        assert_eq!(Tuple(Vec::default()).to_code(), "[]");
    }

    #[test]
    fn union_type() {
        let t = TypeScriptType::from("Circle")
            .with_member("Square")
            .with_member("Triangle");
        assert_eq!(t.to_code(), "Circle | Square | Triangle");
    }

    #[test]
    fn union_type_of_one_type() {
        let t = Union(vec![TypeScriptType::from("Circle")]);
        assert_eq!(t.to_code(), "Circle");
    }

    #[test]
    fn union_type_of_no_types() {
        assert_eq!(Union(Vec::default()).to_code(), "never");
    }

    #[test]
    fn optional_type() {
        let t = TypeScriptType::from("string").to_optional();
        assert_eq!(t.to_code(), "string | undefined");
    }

    #[test]
    fn display_matches_to_code() {
        let t = TypeScriptType::from("Map")
            .with_generic("string")
            .with_generic(TypeScriptType::from("Point").to_optional());
        assert_eq!(format!("{t}"), "Map<string, Point | undefined>");
        assert_eq!(format!("{t}"), t.to_code());
    }
}
