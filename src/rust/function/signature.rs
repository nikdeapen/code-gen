use crate::rust::{
    Receiver, RustType, Var, WithFnGenerics, WithReceiver, WithResult, WithUnsafeFlag,
    WithVarParams,
};
use crate::{CodeBuffer, Expression, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    is_unsafe: bool,
    name: String,
    generics: Vec<Var>,
    receiver: Option<Receiver>,
    params: Vec<Var>,
    result: Option<RustType>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            is_unsafe: false,
            name: name.into(),
            generics: Vec::default(),
            receiver: None,
            params: Vec::default(),
            result: None,
        }
    }
}

impl WithUnsafeFlag for Signature {
    fn is_unsafe(&self) -> bool {
        self.is_unsafe
    }

    fn set_unsafe(&mut self, is_unsafe: bool) {
        self.is_unsafe = is_unsafe;
    }
}

impl WithName for Signature {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithFnGenerics for Signature {
    fn generics(&self) -> &[Var] {
        self.generics.as_slice()
    }

    fn add_generic<V>(&mut self, generic: V)
    where
        V: Into<Var>,
    {
        self.generics.push(generic.into());
    }
}

impl WithReceiver for Signature {
    fn receiver(&self) -> Option<Receiver> {
        self.receiver
    }

    fn set_receiver(&mut self, receiver: Receiver) {
        self.receiver = Some(receiver);
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
    fn result(&self) -> Option<&RustType> {
        self.result.as_ref()
    }

    fn set_result<T>(&mut self, result: T)
    where
        T: Into<RustType>,
    {
        self.result = Some(result.into());
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        self.write_generic_brackets(b);
        b.push('(');
        if let Some(receiver) = self.receiver {
            receiver.write(b);
            if !self.params.is_empty() {
                b.write(", ");
            }
        }
        self.write_params(b);
        b.push(')');
        self.write_result(b);
        self.write_generic_where(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rust::{WithReceiver, WithResult, WithVarParams};

    #[test]
    fn simple_signature() {
        let s = Signature::from("foo");
        assert_eq!(s.to_code(), "foo()");
    }

    #[test]
    fn signature_with_receiver() {
        let s = Signature::from("foo").with_receiver(Receiver::Borrowed);
        assert_eq!(s.to_code(), "foo(&self)");
    }

    #[test]
    fn signature_with_params() {
        let s = Signature::from("foo")
            .with_receiver(Receiver::BorrowedMut)
            .with_param(("x", "u32"));
        assert_eq!(s.to_code(), "foo(&mut self, x: u32)");
    }

    #[test]
    fn signature_with_result() {
        let s = Signature::from("foo").with_result(RustType::from("bool"));
        assert_eq!(s.to_code(), "foo() -> bool");
    }

    #[test]
    fn signature_with_generics() {
        let s = Signature::from("foo")
            .with_generic(Var::from(("T", "Display")))
            .with_param(("value", "T"))
            .with_result(RustType::from("String"));
        assert_eq!(s.to_code(), "foo<T>(value: T) -> String where T: Display");
    }
}
