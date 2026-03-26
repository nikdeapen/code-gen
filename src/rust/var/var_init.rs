use crate::rust::{Var, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A variable initialization statement.
pub struct VarInit {
    mutable: bool,
    var: Var,
    expression: Box<dyn Expression>,
}

impl WithVar for VarInit {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for VarInit {
    fn from(t: (V, E)) -> Self {
        Self {
            mutable: false,
            var: t.0.into(),
            expression: Box::new(t.1),
        }
    }
}

impl VarInit {
    //! Mutable

    /// Sets the mutable flag.
    pub fn set_mutable(&mut self) {
        self.mutable = true;
    }

    /// Sets the mutable flag.
    #[must_use]
    pub fn with_mutable(mut self) -> Self {
        self.set_mutable();
        self
    }
}

impl Statement for VarInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        if self.mutable {
            b.write("let mut ");
        } else {
            b.write("let ");
        }
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn var_init_statement() {
        let v = VarInit::from((("x", "u32"), Literal::from("42")));
        assert_eq!(v.to_code(), "let x: u32 = 42;\n");
    }

    #[test]
    fn var_init_mutable() {
        let v = VarInit::from((("x", "u32"), Literal::from("42"))).with_mutable();
        assert_eq!(v.to_code(), "let mut x: u32 = 42;\n");
    }
}
