use crate::python::{PythonType, Receiver, Var};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature. (ex: `def area(self) -> int`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    is_async: bool,
    name: String,
    receiver: Option<Receiver>,
    params: Vec<Var>,
    result: Option<PythonType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            is_async: false,
            name: name.into(),
            receiver: None,
            params: Vec::default(),
            result: None,
        }
    }
}

impl Signature {
    //! Async

    /// Checks if the function is `async`.
    #[must_use]
    pub fn is_async(&self) -> bool {
        self.is_async
    }

    /// Sets the `async` flag.
    pub fn set_async(&mut self, is_async: bool) {
        self.is_async = is_async;
    }

    /// Sets the `async` flag.
    #[must_use]
    pub fn with_async(mut self, is_async: bool) -> Self {
        self.set_async(is_async);
        self
    }
}

impl WithName for Signature {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Signature {
    //! Receiver

    /// Gets the optional receiver.
    #[must_use]
    pub fn receiver(&self) -> Option<Receiver> {
        self.receiver
    }

    /// Sets the receiver.
    pub fn set_receiver(&mut self, receiver: Receiver) {
        self.receiver = Some(receiver);
    }

    /// Sets the receiver.
    #[must_use]
    pub fn with_receiver(mut self, receiver: Receiver) -> Self {
        self.set_receiver(receiver);
        self
    }
}

impl Signature {
    //! Params

    /// Gets the parameters.
    #[must_use]
    pub fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    /// Adds a parameter.
    pub fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }

    /// Adds a parameter.
    #[must_use]
    pub fn with_param<V>(mut self, param: V) -> Self
    where
        V: Into<Var>,
    {
        self.add_param(param);
        self
    }

    /// Writes the receiver, then the parameters.
    fn write_params(&self, b: &mut CodeBuffer) {
        let mut is_first: bool = true;
        if let Some(receiver) = self.receiver {
            receiver.write(b);
            is_first = false;
        }
        for param in &self.params {
            if !is_first {
                b.write(", ");
            }
            self.write_param(b, param);
            is_first = false;
        }
    }

    /// Writes a parameter.
    ///
    /// # Note
    /// - The default of a parameter with no type has no spaces around its `=`.
    fn write_param(&self, b: &mut CodeBuffer, param: &Var) {
        match (param.python_type(), param.value()) {
            (None, Some(value)) => {
                param.write_name(b);
                b.push('=');
                b.write(value);
            }
            _ => param.write(b),
        }
    }
}

impl Signature {
    //! Result

    /// Gets the optional result.
    #[must_use]
    pub fn result(&self) -> Option<&PythonType> {
        self.result.as_ref()
    }

    /// Sets the result.
    pub fn set_result<T>(&mut self, result: T)
    where
        T: Into<PythonType>,
    {
        self.result = Some(result.into());
    }

    /// Sets the result.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<PythonType>,
    {
        self.set_result(result);
        self
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        if self.is_async {
            b.write("async ");
        }
        b.write("def ");
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        if let Some(result) = &self.result {
            b.write(" -> ");
            result.write(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_signature() {
        let s = Signature::from("foo");
        assert_eq!(s.to_code(), "def foo()");
    }

    #[test]
    fn async_signature() {
        let s = Signature::from("foo").with_async(true);
        assert_eq!(s.to_code(), "async def foo()");
    }

    #[test]
    fn signature_with_receiver() {
        let s = Signature::from("foo").with_receiver(Receiver::Instance);
        assert_eq!(s.to_code(), "def foo(self)");
    }

    #[test]
    fn signature_with_params() {
        let s = Signature::from("foo")
            .with_param(("x", "int"))
            .with_param(("y", "int"));
        assert_eq!(s.to_code(), "def foo(x: int, y: int)");
    }

    #[test]
    fn signature_with_receiver_and_params() {
        let s = Signature::from("foo")
            .with_receiver(Receiver::Class)
            .with_param(("x", "int"));
        assert_eq!(s.to_code(), "def foo(cls, x: int)");
    }

    #[test]
    fn signature_with_result() {
        let s = Signature::from("foo").with_result("bool");
        assert_eq!(s.to_code(), "def foo() -> bool");
    }

    #[test]
    fn signature_with_defaults() {
        let s = Signature::from("foo")
            .with_param(Var::from(("x", "int")).with_value("0"))
            .with_param(Var::from("y").with_value("0"));
        assert_eq!(s.to_code(), "def foo(x: int = 0, y=0)");
    }

    #[test]
    fn signature_with_keyword_only_params() {
        let s = Signature::from("foo")
            .with_receiver(Receiver::Instance)
            .with_param("*")
            .with_param(Var::from(("x", PythonType::from("int").to_optional())).with_value("None"))
            .with_result("None");
        assert_eq!(
            s.to_code(),
            "def foo(self, *, x: int | None = None) -> None"
        );
    }
}
