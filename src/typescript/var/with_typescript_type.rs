use crate::typescript::TypeScriptType;
use crate::{CodeBuffer, Expression};

/// An element with an optional TypeScript type.
pub trait WithTypeScriptType {
    /// Gets the optional TypeScript type.
    fn typescript_type(&self) -> Option<&TypeScriptType>;

    /// Writes the optional TypeScript type.
    fn write_typescript_type(&self, b: &mut CodeBuffer) {
        if let Some(typescript_type) = self.typescript_type() {
            typescript_type.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typescript::Var;

    /// Writes the TypeScript type of the `var`.
    fn write(var: &Var) -> String {
        let mut b: CodeBuffer = CodeBuffer::default();
        var.write_typescript_type(&mut b);
        b.into()
    }

    #[test]
    fn without_type() {
        assert_eq!(write(&Var::from("x")), "");
    }

    #[test]
    fn with_type() {
        assert_eq!(write(&Var::from(("x", "number"))), "number");
    }
}
