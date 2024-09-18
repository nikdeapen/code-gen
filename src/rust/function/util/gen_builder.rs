use crate::rust::Access::Public;
use crate::rust::{
    Access, Function, Receiver, SetFieldExp, Signature, TypeTag, WithAccess, WithReceiver,
    WithResult, WithVarParams,
};
use crate::{Literal, Semi, WithStatements};

/// Generates a builder function for a struct field.
///
/// Example:
///
/// pub fn with_field_name(&mut self, field_name: FieldType) -> Self {
///     self.field_name = field_name;
///     self
/// }
pub fn gen_builder<S, T>(field_name: S, field_type: T) -> Function
where
    S: Into<String>,
    T: Into<TypeTag>,
{
    let field_name: String = field_name.into();
    let fn_name: String = format!("with_{}", field_name);
    gen_custom_builder(Public, fn_name, field_name, field_type)
}

/// Generates a custom builder function for a struct field.
pub fn gen_custom_builder<A, S0, S1, T>(
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
        .with_receiver(Receiver::OwnedMut)
        .with_param((field_name.clone(), field_type))
        .with_result(TypeTag::Named("Self".to_string()));

    Function::from(signature)
        .with_access(access)
        .with_statement(Semi::from(SetFieldExp::from((
            field_name.clone(),
            Literal::from(field_name),
        ))))
        .with_expression_statement(Literal::from("self"))
}
