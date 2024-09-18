use crate::rust::Access::Public;
use crate::rust::{
    Access, Function, Receiver, Signature, TypeTag, WithAccess, WithReceiver, WithResult,
};
use crate::{Literal, WithStatements};

/// Generates a getter function for a field with an expression that wraps the field.
pub fn gen_getter_field_exp<S, T, F>(field_name: S, result_type: T, field_exp: F) -> Function
where
    S: AsRef<str>,
    T: Into<TypeTag>,
    F: Fn(&str) -> String,
{
    let fn_name: String = field_name.as_ref().to_string();
    gen_custom_getter_field_exp(Public, fn_name, field_name, result_type, field_exp)
}

/// Generates a custom getter function for a field with an expression that wraps the field.
pub fn gen_custom_getter_field_exp<A, S0, S1, T, F>(
    access: A,
    fn_name: S0,
    field_name: S1,
    result_type: T,
    field_exp: F,
) -> Function
where
    A: Into<Access>,
    S0: Into<String>,
    S1: AsRef<str>,
    T: Into<TypeTag>,
    F: Fn(&str) -> String,
{
    let signature: Signature = Signature::from(fn_name)
        .with_receiver(Receiver::Borrowed)
        .with_result(result_type);
    let expression: String = field_exp(field_name.as_ref());
    Function::from(signature)
        .with_access(access)
        .with_expression_statement(Literal::from(expression))
}
