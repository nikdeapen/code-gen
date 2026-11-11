use crate::kotlin::{KotlinType, WithKotlinType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated Kotlin type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    kotlin_type: KotlinType,
}

impl<S: Into<String>, T: Into<KotlinType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            kotlin_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithKotlinType for Var {
    fn kotlin_type(&self) -> &KotlinType {
        &self.kotlin_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        b.write(": ");
        self.write_kotlin_type(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_expression() {
        let v = Var::from(("count", "Int"));
        assert_eq!(v.to_code(), "count: Int");
    }

    #[test]
    fn var_with_kotlin_type() {
        let v = Var::from(("names", KotlinType::from("List").with_generic("String")));
        assert_eq!(v.to_code(), "names: List<String>");
    }
}
