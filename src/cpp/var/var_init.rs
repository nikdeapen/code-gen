use crate::cpp::{Var, WithVar};
use crate::{CodeBuffer, Expression, Statement};

/// A variable initialization statement. (ex: `auto count = 0;`)
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
    use crate::cpp::CppType;

    #[test]
    fn var_init_statement() {
        let v = VarInit::from((("count", "std::uint32_t"), Literal::from("42")));
        assert_eq!(v.to_code(), "std::uint32_t count = 42;\n");
    }

    #[test]
    fn var_init_auto() {
        let v = VarInit::from((("count", "auto"), Literal::from("items.size()")));
        assert_eq!(v.to_code(), "auto count = items.size();\n");
    }

    #[test]
    fn var_init_const() {
        let var = Var::from(("name", CppType::from("std::string").to_const()));
        let v = VarInit::from((var, Literal::from("read_name()")));
        let mut b = CodeBuffer::default();
        v.write(&mut b, 1);
        assert_eq!(b.peek(), "    const std::string name = read_name();\n");
    }
}
