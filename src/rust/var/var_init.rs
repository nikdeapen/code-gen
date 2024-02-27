use crate::rust::{TypeTag, Var, WithTypeTag, WithVar};
use crate::{CodeBuffer, Expression, Statement, WithName};

/// A variable initialization statement. `let var_name: type_tag =  expression;\n`
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

impl<E: Expression> WithName for VarInit<E> {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl<E: Expression> WithTypeTag for VarInit<E> {
    fn type_tag(&self) -> &TypeTag {
        self.var.type_tag()
    }
}

impl<E: Expression> WithVar for VarInit<E> {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl<E: Expression> Statement for VarInit<E> {
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
