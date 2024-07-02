use crate::{CodeBuffer, Expression, WithName};

/// An expression that sets a field. `self.field_name = expression`
pub struct SetFieldExp<E: Expression> {
    field_name: String,
    expression: E,
}

impl<S: Into<String>, E: Expression> From<(S, E)> for SetFieldExp<E> {
    fn from(tuple: (S, E)) -> Self {
        Self {
            field_name: tuple.0.into(),
            expression: tuple.1,
        }
    }
}

impl<E: Expression> WithName for SetFieldExp<E> {
    fn name(&self) -> &str {
        self.field_name.as_str()
    }
}

impl<E: Expression> Expression for SetFieldExp<E> {
    fn write(&self, b: &mut CodeBuffer) {
        b.write("self.");
        self.write_name(b);
        b.write(" = ");
        self.expression.write(b);
    }
}
