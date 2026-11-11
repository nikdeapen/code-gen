use crate::kotlin::{KotlinType, Var, WithKotlinType};
use crate::{CodeBuffer, Expression, WithName};

/// A type parameter. (ex: `T` or `T : Comparable<T>`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Generic {
    name: String,
    bound: Option<KotlinType>,
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

impl From<Var> for Generic {
    fn from(var: Var) -> Self {
        Self {
            name: var.name().to_string(),
            bound: Some(var.kotlin_type().clone()),
        }
    }
}

impl Generic {
    //! Properties

    /// Gets the optional upper bound.
    #[must_use]
    pub fn bound(&self) -> Option<&KotlinType> {
        self.bound.as_ref()
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
        if let Some(bound) = &self.bound {
            b.write(" : ");
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
        assert_eq!(Generic::from(String::from("out T")).to_code(), "out T");
    }

    #[test]
    fn generic_with_bound() {
        let generic = Generic::from(Var::from(("T", "Comparable<T>")));
        assert_eq!(generic.to_code(), "T : Comparable<T>");
    }
}
