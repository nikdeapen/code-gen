use crate::csharp::{CSharpType, Generic, Var, WithGenerics, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature.
///
/// # Note
/// - A signature with no result is written with `void`.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    name: String,
    generics: Vec<Generic>,
    params: Vec<Var>,
    result: Option<CSharpType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            generics: Vec::default(),
            params: Vec::default(),
            result: None,
        }
    }
}

impl Signature {
    //! Result

    /// Gets the optional result.
    #[must_use]
    pub fn result(&self) -> Option<&CSharpType> {
        self.result.as_ref()
    }

    /// Sets the result.
    pub fn set_result<T>(&mut self, result: T)
    where
        T: Into<CSharpType>,
    {
        self.result = Some(result.into());
    }

    /// Sets the result.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<CSharpType>,
    {
        self.set_result(result);
        self
    }
}

impl WithName for Signature {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithGenerics for Signature {
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }

    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>,
    {
        self.generics.push(generic.into());
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

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        match &self.result {
            Some(result) => result.write(b),
            None => b.write("void"),
        }
        b.space();
        self.write_name(b);
        self.write_generic_brackets(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_generic_where(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::CSharpPrimitive;

    #[test]
    fn simple_signature() {
        let s: Signature = Signature::from("Run");
        assert_eq!(s.to_code(), "void Run()");
    }

    #[test]
    fn signature_with_params() {
        let s: Signature = Signature::from("Move")
            .with_param(("x", "int"))
            .with_param(("y", "int"));
        assert_eq!(s.to_code(), "void Move(int x, int y)");
    }

    #[test]
    fn signature_with_result() {
        let s: Signature = Signature::from("IsEmpty").with_result(CSharpPrimitive::Boolean);
        assert_eq!(s.to_code(), "bool IsEmpty()");
    }

    #[test]
    fn signature_with_generics() {
        let s: Signature = Signature::from("Create")
            .with_generic(Generic::from("T").with_constraint("new()"))
            .with_param(("count", "int"))
            .with_result(CSharpType::from("List").with_generic("T"));
        assert_eq!(s.to_code(), "List<T> Create<T>(int count) where T : new()");
    }
}
