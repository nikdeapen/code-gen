use crate::rust::{RustType, Var, WithRustType};
use crate::{CodeBuffer, Expression, WithName};

/// A generic parameter, with optional bounds. (ex: `T`, `T: Clone` or `'a`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generic {
    name: String,
    bounds: Option<RustType>,
}

impl From<&str> for Generic {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<String> for Generic {
    fn from(name: String) -> Self {
        Self { name, bounds: None }
    }
}

impl From<Var> for Generic {
    fn from(var: Var) -> Self {
        Self {
            name: var.name().to_string(),
            bounds: Some(var.rust_type().clone()),
        }
    }
}

impl Generic {
    //! Bounds

    /// Gets the optional bounds.
    #[must_use]
    pub fn bounds(&self) -> Option<&RustType> {
        self.bounds.as_ref()
    }
}

impl WithName for Generic {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Expression for Generic {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        if let Some(bounds) = &self.bounds {
            b.write(": ");
            bounds.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_without_bounds() {
        assert_eq!(Generic::from("T").to_code(), "T");
        assert_eq!(Generic::from("'a").to_code(), "'a");
    }

    #[test]
    fn generic_with_bounds() {
        let generic: Generic = Generic::from(Var::from(("T", "Clone")));
        assert_eq!(generic.to_code(), "T: Clone");
    }
}
