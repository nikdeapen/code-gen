use crate::kotlin::{Var, WithVar};
use crate::{CodeBuffer, Statement};

/// A variable initialization statement. (ex: `val x: Int = 1`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInit {
    mutable: bool,
    var: Var,
    expression: String,
}

impl<V: Into<Var>, S: Into<String>> From<(V, S)> for VarInit {
    fn from(tuple: (V, S)) -> Self {
        Self {
            mutable: false,
            var: tuple.0.into(),
            expression: tuple.1.into(),
        }
    }
}

impl VarInit {
    //! Properties

    /// Checks if the variable is a `var`.
    #[must_use]
    pub fn is_mutable(&self) -> bool {
        self.mutable
    }
}

impl VarInit {
    //! Mutable

    /// Makes the variable a `var`.
    pub fn set_mutable(&mut self) {
        self.mutable = true;
    }

    /// Makes the variable a `var`.
    #[must_use]
    pub fn with_mutable(mut self) -> Self {
        self.set_mutable();
        self
    }
}

impl WithVar for VarInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for VarInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        if self.mutable {
            b.write("var ");
        } else {
            b.write("val ");
        }
        self.write_var(b);
        b.write(" = ");
        b.write(self.expression.as_str());
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_init_statement() {
        let v = VarInit::from((("x", "Int"), "42"));
        assert_eq!(v.to_code(), "val x: Int = 42\n");
    }

    #[test]
    fn var_init_mutable() {
        let v = VarInit::from((("x", "Int"), String::from("42"))).with_mutable();
        assert_eq!(v.to_code(), "var x: Int = 42\n");
    }

    #[test]
    fn var_init_indented() {
        let v = VarInit::from((("x", "Int"), "42"));
        let mut b = CodeBuffer::default();
        v.write(&mut b, 1);
        assert_eq!(b.peek(), "    val x: Int = 42\n");
    }
}
