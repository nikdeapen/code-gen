use crate::python::{Field, Function, PythonType, SignatureDec, WithComments, WithDecorators};
use crate::{CodeBuffer, Expression, IsEmpty, Statement, WithName};

/// A class declaration.
///
/// # Note
/// - The fields are written first, then the signature declarations, then the functions.
pub struct Class {
    decorators: Vec<String>,
    name: String,
    bases: Vec<PythonType>,
    comments: Vec<String>,
    fields: Vec<Field>,
    signature_decs: Vec<SignatureDec>,
    functions: Vec<Function>,
}

impl<S: Into<String>> From<S> for Class {
    fn from(name: S) -> Self {
        Self {
            decorators: Vec::default(),
            name: name.into(),
            bases: Vec::default(),
            comments: Vec::default(),
            fields: Vec::default(),
            signature_decs: Vec::default(),
            functions: Vec::default(),
        }
    }
}

impl WithDecorators for Class {
    fn decorators(&self) -> &[String] {
        self.decorators.as_slice()
    }

    fn add_decorator<S>(&mut self, decorator: S)
    where
        S: Into<String>,
    {
        self.decorators.push(decorator.into());
    }
}

impl WithName for Class {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Class {
    //! Bases

    /// Gets the base classes.
    #[must_use]
    pub fn bases(&self) -> &[PythonType] {
        self.bases.as_slice()
    }

    /// Adds a base class. (ex: `Protocol` or `Enum`)
    pub fn add_base<T>(&mut self, base: T)
    where
        T: Into<PythonType>,
    {
        self.bases.push(base.into());
    }

    /// Adds a base class. (ex: `Protocol` or `Enum`)
    #[must_use]
    pub fn with_base<T>(mut self, base: T) -> Self
    where
        T: Into<PythonType>,
    {
        self.add_base(base);
        self
    }

    /// Writes the base classes. (ex: `(Shape, Protocol)`)
    fn write_bases(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.bases.split_first() {
            b.push('(');
            first.write(b);
            for base in rest {
                b.write(", ");
                base.write(b);
            }
            b.push(')');
        }
    }
}

impl WithComments for Class {
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

impl Class {
    //! Fields

    /// Gets the fields.
    #[must_use]
    pub fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    /// Adds a field.
    pub fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>,
    {
        self.fields.push(field.into());
    }

    /// Adds a field.
    #[must_use]
    pub fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<Field>,
    {
        self.add_field(field);
        self
    }

    /// Writes the fields.
    ///
    /// # Note
    /// - A blank line goes after the docstring of the class.
    /// - A blank line goes between two fields when either has a docstring.
    fn write_fields(&self, b: &mut CodeBuffer, level: usize) {
        let mut is_after_docstring: bool = !self.comments.is_empty();
        for (index, field) in self.fields.iter().enumerate() {
            let has_docstring: bool = !field.comments().is_empty();
            if is_after_docstring || (index > 0 && has_docstring) {
                b.end_line();
            }
            field.write(b, level);
            is_after_docstring = has_docstring;
        }
    }
}

impl Class {
    //! Signature Decs

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.signature_decs.as_slice()
    }

    /// Adds a function signature declaration.
    pub fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.signature_decs.push(signature_dec.into());
    }

    /// Adds a function signature declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature_dec: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature_dec);
        self
    }
}

impl Class {
    //! Functions

    /// Gets the functions.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    /// Adds a function.
    pub fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }

    /// Adds a function.
    #[must_use]
    pub fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }

    /// Writes the signature declarations, then the functions.
    ///
    /// # Note
    /// - A blank line goes before each one, unless it starts the body of the class.
    fn write_methods(&self, b: &mut CodeBuffer, level: usize) {
        let mut is_first: bool = self.comments.is_empty() && self.fields.is_empty();
        let signature_decs = self.signature_decs.iter().map(|dec| dec as &dyn Statement);
        let functions = self
            .functions
            .iter()
            .map(|function| function as &dyn Statement);
        for method in signature_decs.chain(functions) {
            if !is_first {
                b.end_line();
            }
            method.write(b, level);
            is_first = false;
        }
    }
}

impl IsEmpty for Class {
    fn is_empty(&self) -> bool {
        self.comments.is_empty()
            && self.fields.is_empty()
            && self.signature_decs.is_empty()
            && self.functions.is_empty()
    }
}

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_decorators(b, level);
        b.indent(level);
        b.write("class ");
        self.write_name(b);
        self.write_bases(b);
        b.push(':');
        b.end_line();
        if self.is_empty() {
            b.line(level + 1, "pass");
        } else {
            self.write_comments(b, level + 1);
            self.write_fields(b, level + 1);
            self.write_methods(b, level + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::python::{Receiver, Signature, Var};

    /// Gets the signature of a method with the `name`.
    fn method(name: &str) -> Signature {
        Signature::from(name).with_receiver(Receiver::Instance)
    }

    #[test]
    fn empty_class() {
        let c = Class::from("Empty");
        assert_eq!(c.to_code(), "class Empty:\n    pass\n");
    }

    #[test]
    fn class_with_bases() {
        let c = Class::from("Circle").with_base("Shape").with_base("ABC");
        assert_eq!(c.to_code(), "class Circle(Shape, ABC):\n    pass\n");
    }

    #[test]
    fn class_with_decorator() {
        let c = Class::from("Point").with_decorator("dataclass(frozen=True)");
        assert_eq!(
            c.to_code(),
            "@dataclass(frozen=True)\nclass Point:\n    pass\n"
        );
    }

    #[test]
    fn class_with_comment() {
        let c = Class::from("Point").with_comment("A point.");
        assert_eq!(c.to_code(), "class Point:\n    \"\"\"A point.\"\"\"\n");
    }

    #[test]
    fn class_with_fields() {
        let c = Class::from("Color")
            .with_base("Enum")
            .with_field(Var::from("RED").with_value("1"))
            .with_field(Var::from("GREEN").with_value("2"));
        assert_eq!(
            c.to_code(),
            "class Color(Enum):\n    RED = 1\n    GREEN = 2\n"
        );
    }

    #[test]
    fn class_with_comment_and_fields() {
        let c = Class::from("Point")
            .with_comment("A point.")
            .with_field(("x", "int"))
            .with_field(("y", "int"));
        assert_eq!(
            c.to_code(),
            "class Point:\n    \"\"\"A point.\"\"\"\n\n    x: int\n    y: int\n"
        );
    }

    #[test]
    fn class_with_documented_fields() {
        let c = Class::from("Point")
            .with_field(Field::from(("w", "int")).with_comment("The w."))
            .with_field(("x", "int"))
            .with_field(("y", "int"))
            .with_field(Field::from(("z", "int")).with_comment("The z."));
        assert_eq!(
            c.to_code(),
            "class Point:\n    w: int\n    \"\"\"The w.\"\"\"\n\n    x: int\n    y: int\n\n    z: int\n    \"\"\"The z.\"\"\"\n"
        );
    }

    #[test]
    fn class_with_signature_decs() {
        let c = Class::from("Shape")
            .with_base("Protocol")
            .with_signature_dec(method("area").with_result("float"))
            .with_signature_dec(method("name").with_result("str"));
        assert_eq!(
            c.to_code(),
            "class Shape(Protocol):\n    def area(self) -> float: ...\n\n    def name(self) -> str: ...\n"
        );
    }

    #[test]
    fn class_with_functions() {
        let c = Class::from("Point")
            .with_function(Function::from(method("x")).with_literal("return self._x"))
            .with_function(Function::from(method("y")).with_literal("return self._y"));
        assert_eq!(
            c.to_code(),
            "class Point:\n    def x(self):\n        return self._x\n\n    def y(self):\n        return self._y\n"
        );
    }

    #[test]
    fn class_with_field_and_function() {
        let c = Class::from("Point")
            .with_field(("x", "int"))
            .with_function(Function::from(method("y")).with_literal("return self._y"));
        assert_eq!(
            c.to_code(),
            "class Point:\n    x: int\n\n    def y(self):\n        return self._y\n"
        );
    }

    #[test]
    fn class_with_all_members() {
        let c = Class::from("Shape")
            .with_decorator("final")
            .with_base("ABC")
            .with_comment("A shape.")
            .with_field(("sides", "int"))
            .with_signature_dec(SignatureDec::from(method("area")).with_decorator("abstractmethod"))
            .with_function(Function::from(method("name")).with_literal("return \"shape\""));
        assert_eq!(
            c.to_code(),
            "@final\nclass Shape(ABC):\n    \"\"\"A shape.\"\"\"\n\n    sides: int\n\n    @abstractmethod\n    def area(self): ...\n\n    def name(self):\n        return \"shape\"\n"
        );
    }

    #[test]
    fn class_indented() {
        let c = Class::from("Point")
            .with_comment("A point.")
            .with_function(Function::from(method("x")).with_literal("return self._x"));
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    class Point:\n        \"\"\"A point.\"\"\"\n\n        def x(self):\n            return self._x\n"
        );
    }
}
