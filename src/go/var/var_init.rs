use crate::go::{Var, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A variable declaration. (ex: `var count uint32 = 0`)
pub struct VarInit {
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for VarInit {
    fn from(tuple: (V, E)) -> Self {
        Self {
            var: tuple.0.into(),
            expression: Box::new(tuple.1),
        }
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
        b.write("var ");
        self.write_var(b);
        b.write(" = ");
        self.expression.write(b);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Literal;

    #[test]
    fn var_init_statement() {
        let v = VarInit::from((("count", "uint32"), Literal::from("0")));
        assert_eq!(v.to_code(), "var count uint32 = 0\n");
    }

    #[test]
    fn var_init_indented() {
        let v = VarInit::from((("count", "uint32"), Literal::from("0")));
        let mut b = CodeBuffer::default();
        v.write(&mut b, 1);
        assert_eq!(b.peek(), "    var count uint32 = 0\n");
    }
}
