use crate::rust::Access::Public;
use crate::rust::{
    Access, FieldExp, Function, Receiver, Signature, TypeTag, WithAccess, WithReceiver, WithResult,
};
use crate::WithStatements;

/// Generates a getter function for a field that is `Copy`.
pub fn gen_getter_copy<S, T>(field_name: S, field_type: T) -> Function
where
    S: Into<String>,
    T: Into<TypeTag>,
{
    let field_name: String = field_name.into();
    let fn_name: String = field_name.clone();
    gen_custom_getter_copy(Public, fn_name, field_name, field_type)
}

/// Generates a custom getter function for a field that is `Copy`.
pub fn gen_custom_getter_copy<A, S0, S1, T>(
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
    let signature: Signature = Signature::from(fn_name)
        .with_receiver(Receiver::Borrowed)
        .with_result(field_type);
    Function::from(signature)
        .with_access(access)
        .with_expression_statement(FieldExp::from(field_name))
}
