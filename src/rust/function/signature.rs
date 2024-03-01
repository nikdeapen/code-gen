use crate::rust::function::with_fn_generics::WithFnGenerics;
use crate::rust::{Receiver, TypeTag, Var, WithReceiver, WithResult, WithVarParams};
use crate::{CodeBuffer, Expression, ToStaticStr, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    name: String,
    generics: Vec<Var>,
    receiver: Option<Receiver>,
    params: Vec<Var>,
    result: Option<TypeTag>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            name: name.into(),
            generics: Vec::default(),
            receiver: None,
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
    fn result(&self) -> Option<&TypeTag> {
        self.result.as_ref()
    }

    fn set_result<T>(&mut self, result: T)
    where
        T: Into<TypeTag>,
    {
        self.result = Some(result.into());
    }
}

impl Expression for Signature {
    fn write(&self, b: &mut CodeBuffer) {
        self.write_name(b);
        if let Some((first, rest)) = self.generics().split_first() {
            b.write("<");
            first.write_name(b);
            for generic in rest {
                b.write(", ");
                generic.write_name(b);
            }
            b.write(">")
        }
        b.write("(");
        if let Some(receiver) = self.receiver {
            b.write(receiver.to_static_str());
            if !self.params.is_empty() {
                b.write(", ");
            }
        }
        self.write_params(b);
        b.write(")");
        self.write_result(b);
        if let Some((first, rest)) = self.generics().split_first() {
            b.write(" where ");
            first.write(b);
            for generic in rest {
                b.write(", ");
                generic.write(b);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::function::with_fn_generics::WithFnGenerics;
    use crate::rust::function::with_result::WithResult;
    use crate::rust::PrimitiveType::{UnsignedInt16, UnsignedInt32, UnsignedInt8};
    use crate::rust::{Receiver, Signature, WithReceiver, WithVarParams};
    use crate::CodeBuffer;

    #[test]
    fn write_empty() {
        let signature: Signature = "myFn".into();
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn()");
    }

    #[test]
    fn write_result() {
        let signature: Signature = Signature::from("myFn").with_result(UnsignedInt8);
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn() -> u8");
    }

    #[test]
    fn write_params() {
        let signature: Signature = Signature::from("myFn").with_param(("one", UnsignedInt8));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(one: u8)");

        let signature: Signature = signature.with_param(("two", UnsignedInt16));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(one: u8, two: u16)");

        let signature: Signature = signature.with_param(("three", UnsignedInt32));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(one: u8, two: u16, three: u32)");
    }

    #[test]
    fn write_receiver() {
        let signature: Signature = Signature::from("myFn").with_receiver(Receiver::BorrowedMut);
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(&mut self)");

        let signature: Signature = signature.with_param(("one", UnsignedInt8));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(&mut self, one: u8)");

        let signature: Signature = signature.with_param(("two", UnsignedInt16));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn(&mut self, one: u8, two: u16)");
    }

    #[test]
    fn write_generics() {
        let signature: Signature = Signature::from("myFn").with_generic(("T", "MyType"));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn<T>() where T: MyType");

        let signature: Signature = signature.with_generic(("B", "BeeType"));
        let result: String = CodeBuffer::display_expression(&signature);
        assert_eq!(result, "myFn<T, B>() where T: MyType, B: BeeType");
    }
}
