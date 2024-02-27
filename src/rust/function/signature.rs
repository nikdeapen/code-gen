use crate::rust::{Receiver, TypeTag, Var, WithReceiver, WithResult, WithVarParams};
use crate::{CodeBuffer, Expression, ToStaticStr, WithName};

/// A function signature.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Signature {
    receiver: Option<Receiver>,
    name: String,
    params: Vec<Var>,
    result: Option<TypeTag>,
}

impl<S: Into<String>> From<S> for Signature {
    fn from(name: S) -> Self {
        Self {
            receiver: None,
            result: None,
            name: name.into(),
            params: Vec::default(),
        }
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
    }
}

#[cfg(test)]
mod tests {
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
}
