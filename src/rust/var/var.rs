use crate::rust::{RustType, WithRustType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated Rust type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    rust_type: RustType,
}

impl<S: Into<String>, T: Into<RustType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            rust_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithRustType for Var {
    fn rust_type(&self) -> &RustType {
        &self.rust_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        b.write(": ");
        self.write_rust_type(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_expression() {
        let v = Var::from(("count", "u32"));
        assert_eq!(v.to_code(), "count: u32");
    }
}
