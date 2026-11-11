use crate::cpp::{CppType, WithCppType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated C++ type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    cpp_type: CppType,
}

impl<S: Into<String>, T: Into<CppType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            cpp_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithCppType for Var {
    fn cpp_type(&self) -> &CppType {
        &self.cpp_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_cpp_type(b);
        b.space();
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_expression() {
        let v = Var::from(("count", "std::uint32_t"));
        assert_eq!(v.to_code(), "std::uint32_t count");
    }

    #[test]
    fn var_with_reference_type() {
        let v = Var::from(("name", CppType::from("std::string").to_const().to_ref()));
        assert_eq!(v.to_code(), "const std::string& name");
    }
}
