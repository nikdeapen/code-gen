use crate::java::{JavaType, Var, WithResult, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    result: Option<JavaType>,
    name: String,
    params: Vec<Var>,
    throws: Vec<JavaType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            result: None,
            name: name.into(),
            params: Vec::default(),
            throws: Vec::default(),
        }
    }
}

impl WithResult for Signature {
    fn result(&self) -> Option<&JavaType> {
        self.result.as_ref()
    }

    fn set_result<T>(&mut self, result: T)
    where
        T: Into<JavaType>,
    {
        self.result = Some(result.into());
    }
}

impl WithName for Signature {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithVarParams for Signature {
    fn params(&self) -> &[Var] {
        self.params.as_slice()
    }

    fn add_param<V>(&mut self, param: V)
    where
        V: Into<Var>,
    {
        self.params.push(param.into());
    }
}

impl Signature {
    //! Throws

    /// Gets the exceptions the function throws.
    #[must_use]
    pub fn throws(&self) -> &[JavaType] {
        self.throws.as_slice()
    }

    /// Adds an exception the function throws.
    pub fn add_throws<T>(&mut self, exception: T)
    where
        T: Into<JavaType>,
    {
        self.throws.push(exception.into());
    }

    /// Adds an exception the function throws.
    #[must_use]
    pub fn with_throws<T>(mut self, exception: T) -> Self
    where
        T: Into<JavaType>,
    {
        self.add_throws(exception);
        self
    }

    /// Writes the `throws` clause, after a space.
    fn write_throws(&self, b: &mut CodeBuffer) {
        if let Some((first, rest)) = self.throws.split_first() {
            b.write(" throws ");
            first.write(b);
            for exception in rest {
                b.write(", ");
                exception.write(b);
            }
        }
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_result(b);
        b.space();
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_throws(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::JavaPrimitive;

    #[test]
    fn simple_signature() {
        let s: Signature = Signature::from("reset");
        assert_eq!(s.to_code(), "void reset()");
    }

    #[test]
    fn signature_with_result() {
        let s: Signature = Signature::from("x").with_result(JavaPrimitive::Int);
        assert_eq!(s.to_code(), "int x()");
    }

    #[test]
    fn signature_with_params() {
        let s: Signature = Signature::from("move")
            .with_param(("x", JavaPrimitive::Int))
            .with_param(("y", JavaPrimitive::Int));
        assert_eq!(s.to_code(), "void move(int x, int y)");
    }

    #[test]
    fn signature_with_throws() {
        let s: Signature = Signature::from("decode")
            .with_param(("bytes", JavaType::from(JavaPrimitive::Byte).to_array()))
            .with_result("Point")
            .with_throws("DecodeException");
        assert_eq!(
            s.to_code(),
            "Point decode(byte[] bytes) throws DecodeException"
        );
    }

    #[test]
    fn signature_with_many_throws() {
        let s: Signature = Signature::from("read")
            .with_throws("IOException")
            .with_throws("DecodeException");
        assert_eq!(
            s.to_code(),
            "void read() throws IOException, DecodeException"
        );
    }
}
