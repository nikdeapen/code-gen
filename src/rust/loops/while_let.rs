use crate::{CodeBuffer, Expression, Statement, WithName, WithStatements};

/// A while-let loop.
pub struct WhileLet {
    variant: String,
    name: String,
    expression: Box<dyn Expression>,
    statements: Vec<Box<dyn Statement>>,
}

impl WhileLet {
    //! Construction

    /// Creates a new while-let loop.
    pub fn new<S0, S1, E>(variant: S0, name: S1, expression: E) -> Self
    where
        S0: Into<String>,
        S1: Into<String>,
        E: 'static + Expression,
    {
        Self {
            variant: variant.into(),
            name: name.into(),
            expression: Box::new(expression),
            statements: Vec::default(),
        }
    }
}

impl WithName for WhileLet {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithStatements for WhileLet {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for WhileLet {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.indent(level);
        b.write("while let ");
        b.write(self.variant.as_str());
        b.write("(");
        self.write_name(b);
        b.write(") = ");
        self.expression.write(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::WhileLet;
    use crate::{CodeBuffer, Literal, WithStatements};

    #[test]
    fn empty() {
        let while_let: WhileLet = WhileLet::new("Some", "value", Literal::from("expression"));
        let result: String = CodeBuffer::display_statement(&while_let);
        assert_eq!(result, "while let Some(value) = expression {}\n");
    }

    #[test]
    fn with_statements() {
        let mut while_let: WhileLet = WhileLet::new("Some", "value", Literal::from("expression"));
        while_let.add_expression_statement(Literal::from("one"));
        while_let.add_expression_statement(Literal::from("two"));
        let result: String = CodeBuffer::display_statement(&while_let);
        assert_eq!(
            result,
            "while let Some(value) = expression {\n\tone\n\ttwo\n}\n"
        );
    }
}
