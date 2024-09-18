use crate::rust::Access::Public;
use crate::rust::{
    Access, Function, Receiver, Signature, TypeTag, Var, WithAccess, WithReceiver, WithResult,
    WithVarParams,
};
use crate::WithStatements;

/// Generates a setter function for a field that can be set with `mem::replace`.
pub fn gen_setter_mem_replace<S, T>(field_name: S, field_type: T) -> Function
where
    S: Into<String>,
    T: Into<TypeTag>,
{
    let field_name: String = field_name.into();
    gen_custom_setter_mem_replace(
        Public,
        format!("set_{}", field_name),
        field_name,
        field_type,
    )
}

/// Generates a custom setter function for a field that can be set with `mem::replace`.
pub fn gen_custom_setter_mem_replace<A, S0, S1, T>(
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

    let field_exp: String = format!(
        "std::mem::replace(&mut self.{}, {})",
        field_name, field_name
    );

    let field_type: TypeTag = field_type.into();
    let field_param: Var = (field_name, field_type.clone()).into();

    let signature: Signature = Signature::from(fn_name)
        .with_receiver(Receiver::BorrowedMut)
        .with_result(field_type)
        .with_param(field_param);

    Function::from(signature)
        .with_access(access)
        .with_literal(field_exp)
}
