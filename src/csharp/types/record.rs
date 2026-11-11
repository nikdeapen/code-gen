use crate::csharp::types::Members;
use crate::csharp::{
    Access, CSharpType, CommentType, Constructor, Field, Function, Modifier, Property,
    SignatureDec, Var, WithAccess, WithAttributes, WithBaseTypes, WithComments, WithConstructors,
    WithFields, WithFunctions, WithModifiers, WithNestedTypes, WithProperties, WithSignatureDecs,
    WithVarParams,
};
use crate::{CodeBuffer, IsEmpty, Statement, WithName};
use std::collections::BTreeSet;

/// A record declaration.
///
/// # Note
/// - Its parameters are its positional parameters, which C# makes properties of.
/// - A record with no members is written with no body.
/// - The members are written as the members of a [Class](crate::csharp::Class) are.
pub struct Record {
    comments: Vec<String>,
    attributes: Vec<String>,
    access: Access,
    modifiers: BTreeSet<Modifier>,
    is_struct: bool,
    name: String,
    params: Vec<Var>,
    base_types: Vec<CSharpType>,
    members: Members,
}

impl<S: Into<String>> From<S> for Record {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            attributes: Vec::default(),
            access: Access::default(),
            modifiers: BTreeSet::default(),
            is_struct: false,
            name: name.into(),
            params: Vec::default(),
            base_types: Vec::default(),
            members: Members::default(),
        }
    }
}

impl Record {
    //! Struct

    /// Checks if the record is a `record struct`.
    #[must_use]
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }

    /// Sets if the record is a `record struct`.
    pub fn set_struct(&mut self, is_struct: bool) {
        self.is_struct = is_struct;
    }

    /// Sets if the record is a `record struct`.
    #[must_use]
    pub fn with_struct(mut self, is_struct: bool) -> Self {
        self.set_struct(is_struct);
        self
    }
}

impl WithComments for Record {
    fn comments(&self) -> &[String] {
        self.comments.as_slice()
    }

    fn add_comment<S>(&mut self, comment: S)
    where
        S: Into<String>,
    {
        self.comments.push(comment.into());
    }
}

impl WithAttributes for Record {
    fn attributes(&self) -> &[String] {
        self.attributes.as_slice()
    }

    fn add_attribute<S>(&mut self, attribute: S)
    where
        S: Into<String>,
    {
        self.attributes.push(attribute.into());
    }
}

impl WithAccess for Record {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithModifiers for Record {
    fn modifiers(&self) -> &BTreeSet<Modifier> {
        &self.modifiers
    }

    fn add_modifier(&mut self, modifier: Modifier) {
        self.modifiers.insert(modifier);
    }
}

impl WithName for Record {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl WithVarParams for Record {
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

impl WithBaseTypes for Record {
    fn base_types(&self) -> &[CSharpType] {
        self.base_types.as_slice()
    }

    fn add_base_type<T>(&mut self, base_type: T)
    where
        T: Into<CSharpType>,
    {
        self.base_types.push(base_type.into());
    }
}

impl WithFields for Record {
    fn fields(&self) -> &[Field] {
        self.members.fields.as_slice()
    }

    fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>,
    {
        self.members.fields.push(field.into());
    }
}

impl WithConstructors for Record {
    fn constructors(&self) -> &[Constructor] {
        self.members.constructors.as_slice()
    }

    fn add_constructor<C>(&mut self, constructor: C)
    where
        C: Into<Constructor>,
    {
        self.members.constructors.push(constructor.into());
    }
}

impl WithProperties for Record {
    fn properties(&self) -> &[Property] {
        self.members.properties.as_slice()
    }

    fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property>,
    {
        self.members.properties.push(property.into());
    }
}

impl WithSignatureDecs for Record {
    fn signature_decs(&self) -> &[SignatureDec] {
        self.members.signature_decs.as_slice()
    }

    fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.members.signature_decs.push(signature_dec.into());
    }
}

impl WithFunctions for Record {
    fn functions(&self) -> &[Function] {
        self.members.functions.as_slice()
    }

    fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.members.functions.push(function.into());
    }
}

impl WithNestedTypes for Record {
    fn nested_types(&self) -> &[Box<dyn Statement>] {
        self.members.nested_types.as_slice()
    }

    fn add_nested_type<S>(&mut self, nested_type: S)
    where
        S: 'static + Statement,
    {
        self.members.nested_types.push(Box::new(nested_type));
    }
}

impl Statement for Record {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::Doc, b, level);
        self.write_attributes(b, level);
        b.indent(level);
        self.write_access(b);
        self.write_modifiers(b);
        if self.is_struct {
            b.write("record struct ");
        } else {
            b.write("record ");
        }
        self.write_name(b);
        if !self.params.is_empty() {
            b.push('(');
            self.write_params(b);
            b.push(')');
        }
        self.write_base_types(b);
        if self.members.is_empty() {
            b.push(';');
            b.end_line();
        } else {
            b.end_line();
            self.members.write(b, level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::csharp::Signature;

    #[test]
    fn empty_record() {
        let r: Record = Record::from("Empty");
        assert_eq!(r.to_code(), "record Empty;\n");
    }

    #[test]
    fn positional_record() {
        let r: Record = Record::from("Circle")
            .with_access(Access::Public)
            .with_modifier(Modifier::Sealed)
            .with_param(("Radius", "double"))
            .with_base_type("Shape");
        assert_eq!(
            r.to_code(),
            "public sealed record Circle(double Radius) : Shape;\n"
        );
    }

    #[test]
    fn record_struct() {
        let r: Record = Record::from("Point")
            .with_access(Access::Public)
            .with_modifier(Modifier::Readonly)
            .with_struct(true)
            .with_param(("X", "int"))
            .with_param(("Y", "int"));
        assert!(r.is_struct());
        assert_eq!(
            r.to_code(),
            "public readonly record struct Point(int X, int Y);\n"
        );
    }

    #[test]
    fn record_with_header() {
        let r: Record = Record::from("Label")
            .with_comment(" A label of a shape.")
            .with_attribute("Serializable")
            .with_param(("Text", "string"))
            .with_base_type("ILabel")
            .with_base_type("IDisposable");
        let mut b: CodeBuffer = CodeBuffer::default();
        r.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// <summary>\n    /// A label of a shape.\n    /// </summary>\n    [Serializable]\n    record Label(string Text) : ILabel, IDisposable;\n"
        );
    }

    #[test]
    fn record_with_members() {
        let r: Record = Record::from("Shape")
            .with_access(Access::Public)
            .with_modifier(Modifier::Abstract)
            .with_field(
                Field::from(("Sides", "int"))
                    .with_access(Access::Public)
                    .with_modifier(Modifier::Const)
                    .with_initializer("4"),
            )
            .with_constructor(Constructor::from("Shape").with_access(Access::Private))
            .with_property(
                Property::from(("Name", "string"))
                    .with_access(Access::Public)
                    .with_expression("\"shape\""),
            )
            .with_signature_dec(
                SignatureDec::from(Signature::from("Area").with_result("double"))
                    .with_access(Access::Public)
                    .with_modifier(Modifier::Abstract),
            )
            .with_function(
                Function::from(Signature::from("Twice").with_result("double"))
                    .with_access(Access::Public)
                    .with_semi("return 2 * Area()"),
            )
            .with_nested_type(
                Record::from("Square")
                    .with_access(Access::Public)
                    .with_modifier(Modifier::Sealed)
                    .with_param(("Side", "double"))
                    .with_base_type("Shape")
                    .with_function(
                        Function::from(Signature::from("Area").with_result("double"))
                            .with_access(Access::Public)
                            .with_modifier(Modifier::Override)
                            .with_semi("return Side * Side"),
                    ),
            );
        assert_eq!(
            r.to_code(),
            "\
public abstract record Shape
{
    public const int Sides = 4;

    private Shape()
    {
    }

    public string Name => \"shape\";

    public abstract double Area();

    public double Twice()
    {
        return 2 * Area();
    }

    public sealed record Square(double Side) : Shape
    {
        public override double Area()
        {
            return Side * Side;
        }
    }
}
"
        );
    }
}
