use crate::c::{CType, Var, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    name: String,
    params: Vec<Var>,
    result: Option<CType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            params: Vec::default(),
            result: None,
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
    //! Result

    /// Gets the optional result. The function returns `void` if there is none.
    #[must_use]
    pub fn result(&self) -> Option<&CType> {
        self.result.as_ref()
    }

    /// Sets the result.
    pub fn set_result<T>(&mut self, result: T)
    where
        T: Into<CType>,
    {
        self.result = Some(result.into());
    }

    /// Sets the result.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<CType>,
    {
        self.set_result(result);
        self
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        match &self.result {
            Some(result) => result.write_before_name(b),
            None => b.write("void "),
        }
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        if let Some(result) = &self.result {
            result.write_after_name(b);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_signature() {
        let s: Signature = Signature::from("foo");
        assert_eq!(s.to_code(), "void foo(void)");
    }

    #[test]
    fn signature_with_params() {
        let s: Signature = Signature::from("foo")
            .with_param(("x", "int32_t"))
            .with_param(("id", CType::from("uint8_t").to_array("16")));
        assert_eq!(s.to_code(), "void foo(int32_t x, uint8_t id[16])");
    }

    #[test]
    fn signature_with_result() {
        let s: Signature = Signature::from("foo").with_result("bool");
        assert_eq!(s.to_code(), "bool foo(void)");
    }

    #[test]
    fn signature_with_pointer_result() {
        let s: Signature =
            Signature::from("foo").with_result(CType::from("char").to_const().to_pointer());
        assert_eq!(s.to_code(), "const char *foo(void)");
    }

    #[test]
    fn signature_with_function_pointer_result() {
        let handler: CType = CType::from("void").with_param(("code", "int")).to_pointer();
        let s: Signature = Signature::from("foo").with_result(handler);
        assert_eq!(s.to_code(), "void (*foo(void))(int code)");
    }
}
