use crate::typescript::TypeScriptType;
use crate::{CodeBuffer, Expression, WithName};

/// A generic parameter, with an optional bound. (ex: `T` or `T extends Shape`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generic {
    name: String,
    bound: Option<TypeScriptType>,
}

impl From<&str> for Generic {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<String> for Generic {
    fn from(name: String) -> Self {
        Self { name, bound: None }
    }
}

impl<S: Into<String>, T: Into<TypeScriptType>> From<(S, T)> for Generic {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            bound: Some(tuple.1.into()),
        }
    }
}

impl WithName for Generic {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Generic {
    //! Bound

    /// Gets the optional bound.
    #[must_use]
    pub fn bound(&self) -> Option<&TypeScriptType> {
        self.bound.as_ref()
    }
}

impl Expression for Generic {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        if let Some(bound) = &self.bound {
            b.write(" extends ");
            bound.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_without_bound() {
        assert_eq!(Generic::from("T").to_code(), "T");
        assert_eq!(Generic::from(String::from("T")).to_code(), "T");
    }

    #[test]
    fn generic_with_bound() {
        let generic: Generic = Generic::from(("T", "Shape"));
        assert_eq!(generic.to_code(), "T extends Shape");
    }
}
