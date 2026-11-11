use crate::python::PythonType;
use crate::{CodeBuffer, Expression, WithName};

/// A name with an optional type & an optional value. (ex: `x`, `x: int` or `x: int = 0`)
///
/// # Note
/// - The value of a parameter is its default.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    python_type: Option<PythonType>,
    value: Option<String>,
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
            python_type: None,
            value: None,
        }
    }
}

impl<S: Into<String>, T: Into<PythonType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            python_type: Some(tuple.1.into()),
            value: None,
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Var {
    //! Type

    /// Gets the optional type.
    #[must_use]
    pub fn python_type(&self) -> Option<&PythonType> {
        self.python_type.as_ref()
    }
}

impl Var {
    //! Value

    /// Gets the optional value.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Sets the value.
    pub fn set_value<S>(&mut self, value: S)
    where
        S: Into<String>,
    {
        self.value = Some(value.into());
    }

    /// Sets the value.
    #[must_use]
    pub fn with_value<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.set_value(value);
        self
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        if let Some(python_type) = &self.python_type {
            b.write(": ");
            python_type.write(b);
        }
        if let Some(value) = &self.value {
            b.write(" = ");
            b.write(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_with_name() {
        let v = Var::from("count");
        assert_eq!(v.to_code(), "count");
    }

    #[test]
    fn var_with_type() {
        let v = Var::from(("count", "int"));
        assert_eq!(v.to_code(), "count: int");
    }

    #[test]
    fn var_with_type_and_value() {
        let v = Var::from(("count", "int")).with_value("0");
        assert_eq!(v.to_code(), "count: int = 0");
    }

    #[test]
    fn var_with_value() {
        let v = Var::from(String::from("RED")).with_value("1");
        assert_eq!(v.to_code(), "RED = 1");
    }
}
