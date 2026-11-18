use crate::{CodeBuffer, Expression, Literal};

/// An `unsafe` block expression. (ex: `unsafe { *ptr }`)
pub struct UnsafeExpression<E: Expression> {
    expression: E,
}

impl<E: Expression> From<E> for UnsafeExpression<E> {
    fn from(expression: E) -> Self {
        Self { expression }
    }
}

impl From<&str> for UnsafeExpression<Literal> {
    fn from(literal: &str) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl From<String> for UnsafeExpression<Literal> {
    fn from(literal: String) -> Self {
        Self::from(Literal::from(literal))
    }
}

impl<E: Expression> Expression for UnsafeExpression<E> {
    fn write(&self, b: &mut CodeBuffer) {
        b.write("unsafe { ");
        self.expression.write(b);
        b.write(" }");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Statement;
    use crate::rust::VarInit;

    #[test]
    fn unsafe_expression_from_str() {
        let e: UnsafeExpression<Literal> = UnsafeExpression::from("*ptr");
        assert_eq!(e.to_code(), "unsafe { *ptr }");
    }

    #[test]
    fn unsafe_expression_from_string() {
        let e: UnsafeExpression<Literal> = UnsafeExpression::from(String::from("*ptr"));
        assert_eq!(e.to_code(), "unsafe { *ptr }");
    }

    #[test]
    fn unsafe_expression_from_expression() {
        let e: UnsafeExpression<Literal> = UnsafeExpression::from(Literal::from("ptr.read()"));
        assert_eq!(e.to_code(), "unsafe { ptr.read() }");
    }

    #[test]
    fn unsafe_expression_in_var_init() {
        let v: VarInit = VarInit::from((("value", "u32"), UnsafeExpression::from("ptr.read()")));
        assert_eq!(v.to_code(), "let value: u32 = unsafe { ptr.read() };\n");
    }
}
