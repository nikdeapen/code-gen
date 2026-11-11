use crate::cpp::{CppType, Var, WithCppType};
use crate::{CodeBuffer, Expression, WithName};

/// A template parameter. (ex: `typename T`, `std::size_t N` or `std::integral T`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generic {
    name: String,
    kind: Option<CppType>,
}

impl From<&str> for Generic {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<String> for Generic {
    fn from(name: String) -> Self {
        Self { name, kind: None }
    }
}

impl From<Var> for Generic {
    fn from(var: Var) -> Self {
        Self {
            name: var.name().to_string(),
            kind: Some(var.cpp_type().clone()),
        }
    }
}

impl Generic {
    //! Kind

    /// Gets the optional kind.
    ///
    /// # Note
    /// - The kind is the type of a value or the concept of a type.
    /// - A parameter with no kind is a `typename`.
    #[must_use]
    pub fn kind(&self) -> Option<&CppType> {
        self.kind.as_ref()
    }
}

impl WithName for Generic {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Expression for Generic {
    fn write(&self, b: &mut CodeBuffer) {
        match &self.kind {
            Some(kind) => kind.write(b),
            None => b.write("typename"),
        }
        b.space();
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_without_kind() {
        assert_eq!(Generic::from("T").to_code(), "typename T");
        assert_eq!(Generic::from(String::from("U")).to_code(), "typename U");
    }

    #[test]
    fn generic_with_value_kind() {
        let generic = Generic::from(Var::from(("N", "std::size_t")));
        assert_eq!(generic.to_code(), "std::size_t N");
    }

    #[test]
    fn generic_with_concept_kind() {
        let generic = Generic::from(Var::from(("T", "std::integral")));
        assert_eq!(generic.to_code(), "std::integral T");
        assert_eq!(generic.kind(), Some(&CppType::from("std::integral")));
    }
}
