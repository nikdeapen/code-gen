use crate::rust::Access::Public;
use crate::rust::{
    Access, FieldExp, Function, Receiver, SetFieldExp, Signature, TypeTag, VarInit, WithAccess,
    WithReceiver, WithResult, WithVarParams,
};
use crate::{Literal, Semi, WithStatements};

/// Generates a setter function for a field that is `Copy`.
pub fn gen_setter_copy<S, T>(field_name: S, field_type: T) -> Function
where
    S: Into<String>,
    T: Into<TypeTag>,
{
    let field_name: String = field_name.into();
    let fn_name: String = format!("set_{}", field_name);
    gen_custom_setter_copy(Public, fn_name, field_name, field_type)
}

/// Generates a custom setter function for a field that is `Copy`.
pub fn gen_custom_setter_copy<A, S0, S1, T>(
    access: A,
    fn_name: S0,
    field_name: S1,
    field_type: T,
) -> Function
where
    A: Into<Access>,
    S0: Into<String>,
    S1: Into<String>,
    T: Into<TypeTag>,
{
    let field_name: String = field_name.into();
    let field_type: TypeTag = field_type.into();
    let signature: Signature = Signature::from(fn_name)
        .with_receiver(Receiver::BorrowedMut)
        .with_param((field_name.clone(), field_type.clone()))
        .with_result(field_type.clone());
    let old_value: &str = "old_value";
    Function::from(signature)
        .with_access(access)
        .with_statement(VarInit::from((
            (old_value, field_type),
            FieldExp::from(field_name.clone()),
        )))
        .with_statement(Semi::from(SetFieldExp::from((
            field_name.clone(),
            Literal::from(field_name),
        ))))
        .with_expression_statement(Literal::from(old_value))
}

#[cfg(test)]
mod tests {
    use crate::rust::PrimitiveType::UnsignedInt32;
    use crate::rust::{gen_setter_copy, Function};
    use crate::CodeBuffer;

    #[test]
    fn fn_gen_getter_copy() {
        let function: Function = gen_setter_copy("field_name", UnsignedInt32);
        let result: String = CodeBuffer::display_statement(&function);
        let expected: String = vec![
            "pub fn set_field_name(&mut self, field_name: u32) -> u32 {",
            "\tlet old_value: u32 = self.field_name;",
            "\tself.field_name = field_name;",
            "\told_value",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(result, expected);
    }
}
