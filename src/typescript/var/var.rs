use crate::typescript::{TypeScriptType, WithTypeScriptType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an optional TypeScript type. (ex: `x`, `x: number` or `x?: number`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    is_optional: bool,
    typescript_type: Option<TypeScriptType>,
}

impl From<&str> for Var {
    fn from(name: &str) -> Self {
        Self::from(name.to_string())
    }
}

impl From<String> for Var {
    fn from(name: String) -> Self {
        Self {
            name,
            is_optional: false,
            typescript_type: None,
        }
    }
}

impl<S: Into<String>, T: Into<TypeScriptType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            is_optional: false,
            typescript_type: Some(tuple.1.into()),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Var {
    //! Optional

    /// Checks if the variable is optional. (ex: `x?: number`)
    #[must_use]
    pub fn is_optional(&self) -> bool {
        self.is_optional
    }

    /// Sets the optional flag.
    pub fn set_optional(&mut self, is_optional: bool) {
        self.is_optional = is_optional;
    }

    /// Sets the optional flag.
    #[must_use]
    pub fn with_optional(mut self, is_optional: bool) -> Self {
        self.set_optional(is_optional);
        self
    }
}

impl WithTypeScriptType for Var {
    fn typescript_type(&self) -> Option<&TypeScriptType> {
        self.typescript_type.as_ref()
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        if self.is_optional {
            b.push('?');
        }
        if let Some(typescript_type) = &self.typescript_type {
            b.write(": ");
            typescript_type.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_without_type() {
        assert_eq!(Var::from("count").to_code(), "count");
        assert_eq!(Var::from(String::from("count")).to_code(), "count");
    }

    #[test]
    fn var_with_type() {
        let v = Var::from(("count", "number"));
        assert_eq!(v.to_code(), "count: number");
    }

    #[test]
    fn optional_var() {
        let v = Var::from(("count", "number")).with_optional(true);
        assert_eq!(v.to_code(), "count?: number");
        assert!(v.is_optional());
    }
}
