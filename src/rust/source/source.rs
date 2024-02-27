use crate::{CodeBuffer, Statement, WithStatements};

/// A source file.
#[derive(Default)]
pub struct Source {
    statements: Vec<Box<dyn Statement>>,
}

impl WithStatements for Source {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Source {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        if let Some((first, rest)) = self.statements.split_first() {
            first.write(b, level);
            for statement in rest {
                b.end_line();
                statement.write(b, level);
            }
        } else {
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Source;
    use crate::{CodeBuffer, Literal, Semi, WithStatements};

    #[test]
    fn write() {
        let source: Source = Source::default();
        let result: String = CodeBuffer::display_statement(&source);
        assert_eq!(result, "\n");

        let source: Source = source.with_statement(Semi::from(Literal::from("one")));
        let result: String = CodeBuffer::display_statement(&source);
        assert_eq!(result, "one;\n");

        let source: Source = source.with_statement(Semi::from(Literal::from("two")));
        let result: String = CodeBuffer::display_statement(&source);
        assert_eq!(result, "one;\n\ntwo;\n");
    }
}
