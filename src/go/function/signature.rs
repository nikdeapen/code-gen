use crate::go::{GoType, Var, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature. (ex: `DecodePoint(bytes []byte) (*Point, error)`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    name: String,
    params: Vec<Var>,
    results: Vec<GoType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            params: Vec::default(),
            results: Vec::default(),
        }
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
    //! Results

    /// Gets the results.
    #[must_use]
    pub fn results(&self) -> &[GoType] {
        self.results.as_slice()
    }

    /// Adds the result.
    pub fn add_result<T>(&mut self, result: T)
    where
        T: Into<GoType>,
    {
        self.results.push(result.into());
    }

    /// Adds the result.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<GoType>,
    {
        self.add_result(result);
        self
    }

    /// Writes the results. (ex: ` uint32` or ` (*Point, error)`)
    fn write_results(&self, b: &mut CodeBuffer) {
        match self.results.as_slice() {
            [] => {}
            [result] => {
                b.space();
                result.write(b);
            }
            [first, rest @ ..] => {
                b.write(" (");
                first.write(b);
                for result in rest {
                    b.write(", ");
                    result.write(b);
                }
                b.push(')');
            }
        }
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_results(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_signature() {
        let s = Signature::from("Reset");
        assert_eq!(s.to_code(), "Reset()");
    }

    #[test]
    fn signature_with_params() {
        let s = Signature::from("NewPoint")
            .with_param(("x", "uint32"))
            .with_param(("y", "uint32"));
        assert_eq!(s.to_code(), "NewPoint(x uint32, y uint32)");
    }

    #[test]
    fn signature_with_result() {
        let s = Signature::from("X").with_result("uint32");
        assert_eq!(s.to_code(), "X() uint32");
    }

    #[test]
    fn signature_with_results() {
        let s = Signature::from("DecodePoint")
            .with_param(("bytes", GoType::from("byte").to_slice()))
            .with_result(GoType::from("Point").to_pointer())
            .with_result("error");
        assert_eq!(s.to_code(), "DecodePoint(bytes []byte) (*Point, error)");
    }
}
