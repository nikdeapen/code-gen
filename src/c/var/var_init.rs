use crate::c::{Var, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A variable initialization statement.
pub struct VarInit {
    var: Var,
    expression: Box<dyn Expression>,
}

impl<V: Into<Var>, E: 'static + Expression> From<(V, E)> for VarInit {
    fn from(t: (V, E)) -> Self {
        Self {
            var: t.0.into(),
            expression: Box::new(t.1),
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
    use crate::c::CType;

    #[test]
    fn var_init_statement() {
        let v: VarInit = VarInit::from((("x", "uint32_t"), Literal::from("42")));
        assert_eq!(v.to_code(), "uint32_t x = 42;\n");
    }

    #[test]
    fn var_init_array() {
        let id: CType = CType::from("uint8_t").to_array("16");
        let v: VarInit = VarInit::from((("id", id), Literal::from("{0}")));
        assert_eq!(v.to_code(), "uint8_t id[16] = {0};\n");
    }

    #[test]
    fn var_init_indented() {
        let v: VarInit = VarInit::from((("x", "uint32_t"), Literal::from("42")));
        let mut b: CodeBuffer = CodeBuffer::default();
        v.write(&mut b, 1);
        assert_eq!(b.peek(), "    uint32_t x = 42;\n");
    }
}
