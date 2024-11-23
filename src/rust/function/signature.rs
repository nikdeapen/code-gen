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

    fn set_unsafe(&mut self) {
        self.is_unsafe = true;
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
        self.receiver = Some(receiver)
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
        b.write("(");
        if let Some(receiver) = self.receiver {
            receiver.write(b);
            if !self.params.is_empty() {
                b.write(", ");
            }
        }
        self.write_params(b);
        b.write(")");
        self.write_result(b);
        self.write_generic_where(b);
    }
}
