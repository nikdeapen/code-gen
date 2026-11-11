use crate::go::{Signature, Var, WithComments, WithSignature};
use crate::{CodeBuffer, Expression, Statement, WithStatements};

/// A function declaration.
///
/// # Note
/// - A function with a receiver is a method. (ex: `func (p *Point) X() uint32`)
pub struct Function {
    comments: Vec<String>,
    receiver: Option<Var>,
    signature: Signature,
    statements: Vec<Box<dyn Statement>>,
}

impl<S: Into<Signature>> From<S> for Function {
    fn from(signature: S) -> Self {
        Self {
            comments: Vec::default(),
            receiver: None,
            signature: signature.into(),
            statements: Vec::default(),
        }
    }
}

impl WithComments for Function {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl Function {
    //! Receiver

    /// Gets the optional receiver.
    #[must_use]
    pub fn receiver(&self) -> Option<&Var> {
        self.receiver.as_ref()
    }

    /// Sets the receiver.
    pub fn set_receiver<V>(&mut self, receiver: V)
    where
        V: Into<Var>,
    {
        self.receiver = Some(receiver.into());
    }

    /// Sets the receiver.
    #[must_use]
    pub fn with_receiver<V>(mut self, receiver: V) -> Self
    where
        V: Into<Var>,
    {
        self.set_receiver(receiver);
        self
    }
}

impl WithSignature for Function {
    fn signature(&self) -> &Signature {
        &self.signature
    }
}

impl WithStatements for Function {
    fn statements(&self) -> &[Box<dyn Statement>] {
        self.statements.as_slice()
    }

    fn add_boxed_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }
}

impl Statement for Function {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        b.write("func ");
        if let Some(receiver) = &self.receiver {
            b.push('(');
            receiver.write(b);
            b.write(") ");
        }
        self.write_signature(b);
        b.space();
        self.write_curly_statement_block(b, level);
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::go::{GoType, WithVarParams};

    #[test]
    fn empty_function() {
        let f = Function::from("main");
        assert_eq!(f.to_code(), "func main() {}\n");
    }

    #[test]
    fn function_with_body() {
        let f = Function::from(
            Signature::from("NewPoint")
                .with_param(("x", "uint32"))
                .with_param(("y", "uint32"))
                .with_result(GoType::from("Point").to_pointer()),
        )
        .with_literal("return &Point{x: x, y: y}");
        assert_eq!(
            f.to_code(),
            "func NewPoint(x uint32, y uint32) *Point {\n    return &Point{x: x, y: y}\n}\n"
        );
    }

    #[test]
    fn function_with_receiver() {
        let f = Function::from(Signature::from("X").with_result("uint32"))
            .with_receiver(("p", GoType::from("Point").to_pointer()))
            .with_literal("return p.x");
        assert_eq!(
            f.to_code(),
            "func (p *Point) X() uint32 {\n    return p.x\n}\n"
        );
    }

    #[test]
    fn function_with_comment() {
        let f = Function::from("main").with_comment(" main runs the program.");
        assert_eq!(f.to_code(), "// main runs the program.\nfunc main() {}\n");
    }
}
