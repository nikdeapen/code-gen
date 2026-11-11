use crate::typescript::{Generic, TypeScriptType, Var, WithGenerics, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature. (ex: `add(a: number, b: number): number`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    name: String,
    generics: Vec<Generic>,
    params: Vec<Var>,
    result: Option<TypeScriptType>,
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

impl Signature {
    //! Result

    /// Gets the optional result.
    #[must_use]
    pub fn result(&self) -> Option<&TypeScriptType> {
        self.result.as_ref()
    }

    /// Sets the `result`.
    pub fn set_result<T>(&mut self, result: T)
    where
        T: Into<TypeScriptType>,
    {
        self.result = Some(result.into());
    }

    /// Sets the `result`.
    #[must_use]
    pub fn with_result<T>(mut self, result: T) -> Self
    where
        T: Into<TypeScriptType>,
    {
        self.set_result(result);
        self
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        self.write_generic_brackets(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        if let Some(result) = &self.result {
            b.write(": ");
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
        assert_eq!(s.to_code(), "foo()");
    }

    #[test]
    fn signature_with_result() {
        let s = Signature::from("foo").with_result("boolean");
        assert_eq!(s.to_code(), "foo(): boolean");
    }

    #[test]
    fn signature_with_generics_and_params() {
        let s = Signature::from("first")
            .with_generic("T")
            .with_param(("values", TypeScriptType::from("T").to_array()))
            .with_result(TypeScriptType::from("T").to_optional());
        assert_eq!(s.to_code(), "first<T>(values: T[]): T | undefined");
    }
}
