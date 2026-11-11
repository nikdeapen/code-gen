use crate::go::{GoType, WithGoType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with its Go type. (ex: `count uint32`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    go_type: GoType,
}

impl<S: Into<String>, T: Into<GoType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            go_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithGoType for Var {
    fn go_type(&self) -> &GoType {
        &self.go_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        b.space();
        self.write_go_type(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_expression() {
        let v = Var::from(("count", "uint32"));
        assert_eq!(v.to_code(), "count uint32");
    }

    #[test]
    fn var_with_a_go_type() {
        let v = Var::from(("bytes", GoType::from("byte").to_slice()));
        assert_eq!(v.to_code(), "bytes []byte");
    }
}
