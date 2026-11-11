use crate::csharp::{CSharpType, WithCSharpType};
use crate::{CodeBuffer, Expression, WithName};

/// A name with an associated C# type.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Var {
    name: String,
    csharp_type: CSharpType,
}

impl<S: Into<String>, T: Into<CSharpType>> From<(S, T)> for Var {
    fn from(tuple: (S, T)) -> Self {
        Self {
            name: tuple.0.into(),
            csharp_type: tuple.1.into(),
        }
    }
}

impl WithName for Var {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithCSharpType for Var {
    fn csharp_type(&self) -> &CSharpType {
        &self.csharp_type
    }
}

impl Expression for Var {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_csharp_type(b);
        b.space();
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_expression() {
        let v: Var = Var::from(("count", "int"));
        assert_eq!(v.to_code(), "int count");
    }
}
