use crate::rust::Var;
use crate::{CodeBuffer, Expression, Statement};

/// A variable initialization statement.
pub struct VarInit<E: Expression> {
    var: Var,
    expression: E,
}

impl<V: Into<Var>, E: Expression> From<(V, E)> for VarInit<E> {
    fn from(tuple: (V, E)) -> Self {
        Self {
            var: tuple.0.into(),
            expression: tuple.1,
        }
    }
}

impl<E: Expression> Statement for VarInit<E> {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("let ");
        self.var.write(b);
        b.write(" = ");
        self.expression.write(b);
        b.write(";");
        b.end_line();
    }
}
