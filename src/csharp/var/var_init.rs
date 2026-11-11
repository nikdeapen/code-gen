use crate::csharp::{Var, WithVar};
use crate::{CodeBuffer, Statement};

/// A variable initialization statement.
///
/// # Note
/// - A variable of the type `var` has its type inferred.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct VarInit {
    var: Var,
    expression: String,
}

impl<V: Into<Var>, S: Into<String>> From<(V, S)> for VarInit {
    fn from(tuple: (V, S)) -> Self {
        Self {
            var: tuple.0.into(),
            expression: tuple.1.into(),
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
        b.write(self.expression.as_str());
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn var_init_statement() {
        let v: VarInit = VarInit::from((("x", "int"), "42"));
        assert_eq!(v.to_code(), "int x = 42;\n");
    }

    #[test]
    fn var_init_inferred() {
        let v: VarInit = VarInit::from((("x", "var"), "42"));
        let mut b: CodeBuffer = CodeBuffer::default();
        v.write(&mut b, 1);
        assert_eq!(b.peek(), "    var x = 42;\n");
    }
}
