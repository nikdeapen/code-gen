use crate::cpp::CppType::{Const, Generic, Named, Pointer, Ref, RvalueRef};
use crate::{CodeBuffer, Expression};
use std::fmt::{Display, Formatter};

/// A C++ type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum CppType {
    /// A named type. (ex: `std::string`)
    Named(String),

    /// A `const` type.
    Const(Box<CppType>),

    /// A pointer type.
    Pointer(Box<CppType>),

    /// An lvalue reference type.
    Ref(Box<CppType>),

    /// An rvalue reference type.
    RvalueRef(Box<CppType>),

    /// A type with template arguments. (ex: `std::array<T, 16>`)
    ///
    /// # Note
    /// - An argument that is a value is a named type. (ex: the `16`)
    Generic {
        base: Box<CppType>,
        generics: Vec<CppType>,
    },
}

impl<S: Into<String>> From<S> for CppType {
    fn from(name: S) -> Self {
        Named(name.into())
    }
}

impl CppType {
    //! Const Types

    /// Converts the type to a `const` type of itself.
    #[must_use]
    pub fn to_const(self) -> Self {
        match self {
            Const(_) => self,
            base => Const(Box::new(base)),
        }
    }
}

impl CppType {
    //! Pointer Types

    /// Converts the type to a pointer type of itself.
    #[must_use]
    pub fn to_pointer(self) -> Self {
        Pointer(Box::new(self))
    }
}

impl CppType {
    //! Reference Types

    /// Converts the type to an lvalue reference type of itself.
    #[must_use]
    pub fn to_ref(self) -> Self {
        Ref(Box::new(self))
    }

    /// Converts the type to an rvalue reference type of itself.
    #[must_use]
    pub fn to_rvalue_ref(self) -> Self {
        RvalueRef(Box::new(self))
    }
}

impl CppType {
    //! Generic Types

    /// Adds the template argument.
    #[must_use]
    pub fn with_generic<T>(self, generic: T) -> Self
    where
        T: Into<CppType>,
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

impl Expression for CppType {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Named(name) => b.write(name.as_str()),
            Const(base) => {
                if matches!(base.as_ref(), Pointer(_)) {
                    base.write(b);
                    b.write(" const");
                } else {
                    b.write("const ");
                    base.write(b);
                }
            }
            Pointer(base) => {
                base.write(b);
                b.push('*');
            }
            Ref(base) => {
                base.write(b);
                b.push('&');
            }
            RvalueRef(base) => {
                base.write(b);
                b.write("&&");
            }
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
        }
    }
}

impl Display for CppType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_code().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_type() {
        let t = CppType::from("std::string");
        assert_eq!(t.to_code(), "std::string");
    }

    #[test]
    fn const_type() {
        let t = CppType::from("std::string").to_const();
        assert_eq!(t.to_code(), "const std::string");
    }

    #[test]
    fn const_type_once() {
        let t = CppType::from("int").to_const().to_const();
        assert_eq!(t.to_code(), "const int");
    }

    #[test]
    fn pointer_type() {
        let t = CppType::from("Node").to_pointer();
        assert_eq!(t.to_code(), "Node*");
    }

    #[test]
    fn pointer_to_const_type() {
        let t = CppType::from("std::uint8_t").to_const().to_pointer();
        assert_eq!(t.to_code(), "const std::uint8_t*");
    }

    #[test]
    fn const_pointer_type() {
        let t = CppType::from("Node").to_pointer().to_const();
        assert_eq!(t.to_code(), "Node* const");
    }

    #[test]
    fn reference_type() {
        let t = CppType::from("std::string").to_ref();
        assert_eq!(t.to_code(), "std::string&");
    }

    #[test]
    fn const_reference_type() {
        let t = CppType::from("std::string").to_const().to_ref();
        assert_eq!(t.to_code(), "const std::string&");
    }

    #[test]
    fn rvalue_reference_type() {
        let t = CppType::from("std::string").to_rvalue_ref();
        assert_eq!(t.to_code(), "std::string&&");
    }

    #[test]
    fn generic_type() {
        let t = CppType::from("std::vector").with_generic("std::string");
        assert_eq!(t.to_code(), "std::vector<std::string>");
    }

    #[test]
    fn generic_type_multiple() {
        let t = CppType::from("std::map")
            .with_generic("std::string")
            .with_generic(CppType::from("std::optional").with_generic("std::uint32_t"));
        assert_eq!(
            t.to_code(),
            "std::map<std::string, std::optional<std::uint32_t>>"
        );
    }

    #[test]
    fn generic_type_with_value() {
        let t = CppType::from("std::array")
            .with_generic("std::uint8_t")
            .with_generic("16");
        assert_eq!(t.to_code(), "std::array<std::uint8_t, 16>");
    }

    #[test]
    fn generic_type_without_generics() {
        let t = Generic {
            base: Box::new(CppType::from("Point")),
            generics: Vec::default(),
        };
        assert_eq!(t.to_code(), "Point");
    }

    #[test]
    fn display_matches_to_code() {
        let t = CppType::from("std::span")
            .with_generic(CppType::from("std::uint8_t").to_const())
            .to_const()
            .to_ref();
        assert_eq!(format!("{t}"), "const std::span<const std::uint8_t>&");
        assert_eq!(format!("{t}"), t.to_code());
    }
}
