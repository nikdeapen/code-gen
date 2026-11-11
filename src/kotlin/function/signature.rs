use crate::kotlin::{Generic, KotlinType, Var, WithGenerics, WithResult, WithVarParams};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    generics: Vec<Generic>,
    name: String,
    params: Vec<Var>,
    result: Option<KotlinType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            generics: Vec::default(),
            name: name.into(),
            params: Vec::default(),
            result: None,
        }
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

impl WithResult for Signature {
    fn result(&self) -> Option<&KotlinType> {
        self.result.as_ref()
    }

    fn set_result<T>(&mut self, result: T)
    where
        T: Into<KotlinType>,
    {
        self.result = Some(result.into());
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        if !self.generics.is_empty() {
            self.write_generic_brackets(b);
            b.space();
        }
        self.write_name(b);
        b.push('(');
        self.write_params(b);
        b.push(')');
        self.write_result(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_signature() {
        let s = Signature::from("reset");
        assert_eq!(s.to_code(), "reset()");
    }

    #[test]
    fn signature_with_params_and_result() {
        let s = Signature::from("add")
            .with_param(("x", "Int"))
            .with_param(("y", "Int"))
            .with_result("Int");
        assert_eq!(s.to_code(), "add(x: Int, y: Int): Int");
    }

    #[test]
    fn signature_with_generics() {
        let s = Signature::from("largest")
            .with_generic(Var::from(("T", "Comparable<T>")))
            .with_param(("values", KotlinType::from("List").with_generic("T")))
            .with_result(KotlinType::from("T").to_nullable());
        assert_eq!(
            s.to_code(),
            "<T : Comparable<T>> largest(values: List<T>): T?"
        );
    }
}
