use crate::java::{JavaType, WithJavaType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated Java type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    java_type: JavaType,
}

impl<S: Into<String>, T: Into<JavaType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            java_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithJavaType for Var {
    fn java_type(&self) -> &JavaType {
        &self.java_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_java_type(b);
        b.space();
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::JavaPrimitive;

    #[test]
    fn var_expression() {
        let v: Var = Var::from(("count", JavaPrimitive::Int));
        assert_eq!(v.to_code(), "int count");
    }

    #[test]
    fn var_with_a_named_type() {
        let v: Var = Var::from(("name", "String"));
        assert_eq!(v.to_code(), "String name");
    }
}
