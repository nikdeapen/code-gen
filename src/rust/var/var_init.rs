use crate::rust::{Var, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A variable initialization statement.
pub struct VarInit {
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
            var: t.0.into(),
            expression: Box::new(t.1),
        }
    }
}

impl Statement for VarInit {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("let ");
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.write(";");
        b.end_line();
    }
}
