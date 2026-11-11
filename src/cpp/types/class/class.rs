use crate::cpp::{
    Access, CommentType, ConstInit, Constructor, CppType, Field, Function, Generic, SignatureDec,
    TypeDec, WithAccess, WithComments, WithGenerics,
};
use crate::{CodeBuffer, Expression, IsEmpty, Statement, WithName};

/// A class declaration.
///
/// # Note
/// - The members are written in a section for each access level. The public section is first.
/// - A section holds its type declarations, constants, constructors, functions, then fields.
/// - The public section of a `struct` has no label.
pub struct Class {
    comments: Vec<String>,
    generics: Vec<Generic>,
    is_struct: bool,
    name: String,
    bases: Vec<CppType>,
    type_decs: Vec<TypeDec>,
    constants: Vec<ConstInit>,
    constructors: Vec<Constructor>,
    has_virtual_destructor: bool,
    signature_decs: Vec<SignatureDec>,
    functions: Vec<Function>,
    fields: Vec<Field>,
}

/// The writer of the section of a class with one access level.
struct Section<'a> {
    b: &'a mut CodeBuffer,

    /// The indent level of the class.
    level: usize,

    access: Access,

    /// Whether the section starts with its label. (ex: `public:`)
    has_label: bool,

    /// Whether no section of the class is written before this one.
    is_first: bool,

    is_empty: bool,
}

impl Section<'_> {
    //! Write

    /// Writes the `members` that have the access level of the section.
    ///
    /// # Note
    /// - A blank line goes between two members, unless both are on one line.
    /// - A member is on one line if its kind is & it has no comments.
    fn write_members<M>(&mut self, members: &[M], is_one_line_kind: bool)
    where
        M: Statement + WithAccess + WithComments,
    {
        let access: Access = self.access;
        let mut is_after_one_line: bool = false;
        for member in members.iter().filter(|member| member.access() == access) {
            let is_one_line: bool = is_one_line_kind && member.comments().is_empty();
            self.write_gap(is_after_one_line && is_one_line);
            member.write(self.b, self.level + 1);
            is_after_one_line = is_one_line;
        }
    }

    /// Writes the defaulted virtual destructor of the class with the `name`.
    fn write_virtual_destructor(&mut self, name: &str) {
        self.write_gap(false);
        self.b.indent(self.level + 1);
        self.b.write("virtual ~");
        self.b.write(name);
        self.b.write("() = default;");
        self.b.end_line();
    }

    /// Writes what goes before a member.
    ///
    /// # Note
    /// - The first member goes after the label of the section.
    /// - Any other member goes after a blank line, unless `is_tight`.
    fn write_gap(&mut self, is_tight: bool) {
        if self.is_empty {
            if !self.is_first {
                self.b.end_line();
            }
            if self.has_label {
                self.b.indent(self.level);
                self.access.write(self.b);
                self.b.push(':');
                self.b.end_line();
            }
            self.is_empty = false;
        } else if !is_tight {
            self.b.end_line();
        }
    }
}

impl<S: Into<String>> From<S> for Class {
    fn from(name: S) -> Self {
        Self {
            comments: Vec::default(),
            generics: Vec::default(),
            is_struct: false,
            name: name.into(),
            bases: Vec::default(),
            type_decs: Vec::default(),
            constants: Vec::default(),
            constructors: Vec::default(),
            has_virtual_destructor: false,
            signature_decs: Vec::default(),
            functions: Vec::default(),
            fields: Vec::default(),
        }
    }
}

impl WithComments for Class {
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

impl WithGenerics for Class {
    fn generics(&self) -> &[Generic] {
        self.generics.as_slice()
    }

    fn add_generic<G>(&mut self, generic: G)
    where
        G: Into<Generic>,
    {
        self.generics.push(generic.into());
    }
}

impl Class {
    //! Struct

    /// Checks if the class is declared as a `struct`.
    #[must_use]
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }

    /// Sets if the class is declared as a `struct`.
    pub fn set_struct(&mut self, is_struct: bool) {
        self.is_struct = is_struct;
    }

    /// Sets if the class is declared as a `struct`.
    #[must_use]
    pub fn with_struct(mut self, is_struct: bool) -> Self {
        self.set_struct(is_struct);
        self
    }
}

impl WithName for Class {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Class {
    //! Bases

    /// Gets the public base classes.
    #[must_use]
    pub fn bases(&self) -> &[CppType] {
        self.bases.as_slice()
    }

    /// Adds the public base class.
    pub fn add_base<T>(&mut self, base: T)
    where
        T: Into<CppType>,
    {
        self.bases.push(base.into());
    }

    /// Adds the public base class.
    #[must_use]
    pub fn with_base<T>(mut self, base: T) -> Self
    where
        T: Into<CppType>,
    {
        self.add_base(base);
        self
    }

    /// Writes the base classes. (ex: ` : public Shape, public Named`)
    fn write_bases(&self, b: &mut CodeBuffer) {
        for (index, base) in self.bases.iter().enumerate() {
            b.write(if index == 0 {
                " : public "
            } else {
                ", public "
            });
            base.write(b);
        }
    }
}

impl Class {
    //! Type Declarations

    /// Gets the type declarations.
    #[must_use]
    pub fn type_decs(&self) -> &[TypeDec] {
        self.type_decs.as_slice()
    }

    /// Adds the type declaration.
    pub fn add_type_dec<D>(&mut self, type_dec: D)
    where
        D: Into<TypeDec>,
    {
        self.type_decs.push(type_dec.into());
    }

    /// Adds the type declaration.
    #[must_use]
    pub fn with_type_dec<D>(mut self, type_dec: D) -> Self
    where
        D: Into<TypeDec>,
    {
        self.add_type_dec(type_dec);
        self
    }
}

impl Class {
    //! Constants

    /// Gets the constants.
    #[must_use]
    pub fn constants(&self) -> &[ConstInit] {
        self.constants.as_slice()
    }

    /// Adds the constant.
    pub fn add_constant(&mut self, constant: ConstInit) {
        self.constants.push(constant);
    }

    /// Adds the constant.
    #[must_use]
    pub fn with_constant(mut self, constant: ConstInit) -> Self {
        self.add_constant(constant);
        self
    }
}

impl Class {
    //! Constructors

    /// Gets the constructors.
    #[must_use]
    pub fn constructors(&self) -> &[Constructor] {
        self.constructors.as_slice()
    }

    /// Adds the constructor.
    pub fn add_constructor<C>(&mut self, constructor: C)
    where
        C: Into<Constructor>,
    {
        self.constructors.push(constructor.into());
    }

    /// Adds the constructor.
    #[must_use]
    pub fn with_constructor<C>(mut self, constructor: C) -> Self
    where
        C: Into<Constructor>,
    {
        self.add_constructor(constructor);
        self
    }
}

impl Class {
    //! Virtual Destructor

    /// Checks if the class has a public defaulted virtual destructor.
    #[must_use]
    pub fn has_virtual_destructor(&self) -> bool {
        self.has_virtual_destructor
    }

    /// Sets if the class has a public defaulted virtual destructor. (ex: an interface)
    pub fn set_virtual_destructor(&mut self, has_virtual_destructor: bool) {
        self.has_virtual_destructor = has_virtual_destructor;
    }

    /// Sets if the class has a public defaulted virtual destructor. (ex: an interface)
    #[must_use]
    pub fn with_virtual_destructor(mut self, has_virtual_destructor: bool) -> Self {
        self.set_virtual_destructor(has_virtual_destructor);
        self
    }
}

impl Class {
    //! Signature Declarations

    /// Gets the function signature declarations.
    #[must_use]
    pub fn signature_decs(&self) -> &[SignatureDec] {
        self.signature_decs.as_slice()
    }

    /// Adds the function signature declaration.
    pub fn add_signature_dec<S>(&mut self, signature_dec: S)
    where
        S: Into<SignatureDec>,
    {
        self.signature_decs.push(signature_dec.into());
    }

    /// Adds the function signature declaration.
    #[must_use]
    pub fn with_signature_dec<S>(mut self, signature_dec: S) -> Self
    where
        S: Into<SignatureDec>,
    {
        self.add_signature_dec(signature_dec);
        self
    }
}

impl Class {
    //! Functions

    /// Gets the functions.
    #[must_use]
    pub fn functions(&self) -> &[Function] {
        self.functions.as_slice()
    }

    /// Adds the function.
    pub fn add_function<F>(&mut self, function: F)
    where
        F: Into<Function>,
    {
        self.functions.push(function.into());
    }

    /// Adds the function.
    #[must_use]
    pub fn with_function<F>(mut self, function: F) -> Self
    where
        F: Into<Function>,
    {
        self.add_function(function);
        self
    }
}

impl Class {
    //! Fields

    /// Gets the fields.
    #[must_use]
    pub fn fields(&self) -> &[Field] {
        self.fields.as_slice()
    }

    /// Adds the field.
    pub fn add_field<F>(&mut self, field: F)
    where
        F: Into<Field>,
    {
        self.fields.push(field.into());
    }

    /// Adds the field.
    #[must_use]
    pub fn with_field<F>(mut self, field: F) -> Self
    where
        F: Into<Field>,
    {
        self.add_field(field);
        self
    }
}

impl IsEmpty for Class {
    fn is_empty(&self) -> bool {
        self.type_decs.is_empty()
            && self.constants.is_empty()
            && self.constructors.is_empty()
            && !self.has_virtual_destructor
            && self.signature_decs.is_empty()
            && self.functions.is_empty()
            && self.fields.is_empty()
    }
}

impl Statement for Class {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        self.write_generics(b, level);
        b.indent(level);
        b.write(if self.is_struct { "struct " } else { "class " });
        self.write_name(b);
        self.write_bases(b);
        b.write(" {");
        if self.is_empty() {
            b.write("};");
            b.end_line();
        } else {
            b.end_line();
            let mut is_first: bool = true;
            for access in [Access::Public, Access::Protected, Access::Private] {
                let mut section: Section = Section {
                    b: &mut *b,
                    level,
                    access,
                    has_label: !(self.is_struct && access == Access::Public),
                    is_first,
                    is_empty: true,
                };
                section.write_members(self.type_decs(), true);
                section.write_members(self.constants(), true);
                section.write_members(self.constructors(), false);
                if self.has_virtual_destructor && access == Access::Public {
                    section.write_virtual_destructor(self.name());
                }
                section.write_members(self.signature_decs(), true);
                section.write_members(self.functions(), false);
                section.write_members(self.fields(), true);
                is_first = is_first && section.is_empty;
            }
            b.line(level, "};");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpp::{Modifier, Signature, WithModifiers, WithVarParams};
    use crate::{Literal, WithStatements};

    #[test]
    fn empty_class() {
        let c = Class::from("Empty");
        assert!(c.is_empty());
        assert!(!c.is_struct());
        assert_eq!(c.to_code(), "class Empty {};\n");
    }

    #[test]
    fn empty_struct() {
        let c = Class::from("Empty").with_struct(true);
        assert!(c.is_struct());
        assert_eq!(c.to_code(), "struct Empty {};\n");
    }

    #[test]
    fn class_with_comment_generics_and_bases() {
        let c = Class::from("Wrapper")
            .with_comment(" A wrapper of a value.")
            .with_generic("T")
            .with_base("Shape")
            .with_base(CppType::from("Named").with_generic("T"));
        assert_eq!(c.bases().len(), 2);
        assert_eq!(
            c.to_code(),
            "/// A wrapper of a value.\ntemplate <typename T>\nclass Wrapper : public Shape, public Named<T> {};\n"
        );
    }

    #[test]
    fn class_with_members_in_order() {
        let x = Function::from(
            Signature::from("x")
                .with_result("int")
                .with_modifier(Modifier::Const),
        )
        .with_access(Access::Public)
        .with_semi("return x_");
        let size = ConstInit::from((("SIZE", "int"), Literal::from("4")))
            .with_modifier(Modifier::Static)
            .with_access(Access::Public);
        let c = Class::from("Point")
            .with_field(("x_", "int"))
            .with_function(x)
            .with_signature_dec(SignatureDec::from("reset").with_access(Access::Public))
            .with_constructor(Constructor::from("Point").with_access(Access::Public))
            .with_constant(size)
            .with_type_dec(TypeDec::from(("Value", "int")).with_access(Access::Public));
        assert!(!c.is_empty());
        assert_eq!(c.type_decs().len(), 1);
        assert_eq!(c.constants().len(), 1);
        assert_eq!(c.constructors().len(), 1);
        assert_eq!(c.signature_decs().len(), 1);
        assert_eq!(c.functions().len(), 1);
        assert_eq!(c.fields().len(), 1);
        assert_eq!(
            c.to_code(),
            "class Point {\npublic:\n    using Value = int;\n\n    static constexpr int SIZE = 4;\n\n    Point() {}\n\n    void reset();\n\n    int x() const {\n        return x_;\n    }\n\nprivate:\n    int x_;\n};\n"
        );
    }

    #[test]
    fn class_with_each_section() {
        let c = Class::from("Shape")
            .with_field(Field::from(("id_", "int")))
            .with_field(Field::from(("sides_", "int")).with_access(Access::Protected))
            .with_signature_dec(SignatureDec::from("draw").with_access(Access::Public));
        let mut b = CodeBuffer::default();
        c.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    class Shape {\n    public:\n        void draw();\n\n    protected:\n        int sides_;\n\n    private:\n        int id_;\n    };\n"
        );
    }

    #[test]
    fn class_with_only_private_members() {
        let c = Class::from("Point")
            .with_field(("x_", "int"))
            .with_field(("y_", "int"));
        assert_eq!(
            c.to_code(),
            "class Point {\nprivate:\n    int x_;\n    int y_;\n};\n"
        );
    }

    #[test]
    fn struct_with_public_fields() {
        let c = Class::from("Point")
            .with_struct(true)
            .with_field(Field::from(("x", "int")).with_access(Access::Public))
            .with_field(Field::from(("y", "int")).with_access(Access::Public));
        assert_eq!(c.to_code(), "struct Point {\n    int x;\n    int y;\n};\n");
    }

    #[test]
    fn struct_with_private_members() {
        let c = Class::from("Point")
            .with_struct(true)
            .with_field(Field::from(("x", "int")).with_access(Access::Public))
            .with_field(("cache_", "int"));
        assert_eq!(
            c.to_code(),
            "struct Point {\n    int x;\n\nprivate:\n    int cache_;\n};\n"
        );
        let c = Class::from("Point")
            .with_struct(true)
            .with_field(("cache_", "int"));
        assert_eq!(
            c.to_code(),
            "struct Point {\nprivate:\n    int cache_;\n};\n"
        );
    }

    #[test]
    fn class_with_virtual_destructor() {
        let x = Signature::from("x")
            .with_result("int")
            .with_modifier(Modifier::Virtual)
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::PureVirtual);
        let c = Class::from("PointView")
            .with_virtual_destructor(true)
            .with_constructor(Constructor::from("PointView").with_access(Access::Public))
            .with_signature_dec(SignatureDec::from(x).with_access(Access::Public));
        assert!(c.has_virtual_destructor());
        assert_eq!(
            c.to_code(),
            "class PointView {\npublic:\n    PointView() {}\n\n    virtual ~PointView() = default;\n\n    virtual int x() const = 0;\n};\n"
        );
    }

    #[test]
    fn class_with_only_a_virtual_destructor() {
        let c = Class::from("Service")
            .with_virtual_destructor(true)
            .with_field(("calls_", "int"));
        assert!(!c.is_empty());
        assert_eq!(
            c.to_code(),
            "class Service {\npublic:\n    virtual ~Service() = default;\n\nprivate:\n    int calls_;\n};\n"
        );
    }

    #[test]
    fn blank_lines_between_members() {
        let equals = Signature::from("operator==")
            .with_param(("other", CppType::from("Point").to_const().to_ref()))
            .with_result("bool")
            .with_modifier(Modifier::Const)
            .with_modifier(Modifier::Default);
        let c = Class::from("Point")
            .with_struct(true)
            .with_signature_dec(SignatureDec::from(equals).with_access(Access::Public))
            .with_signature_dec(SignatureDec::from("reset").with_access(Access::Public))
            .with_signature_dec(
                SignatureDec::from("clear")
                    .with_comment(" Clears the point.")
                    .with_access(Access::Public),
            )
            .with_function(Function::from("a").with_access(Access::Public))
            .with_function(Function::from("b").with_access(Access::Public))
            .with_field(Field::from(("x", "int")).with_access(Access::Public))
            .with_field(
                Field::from(("y", "int"))
                    .with_comment(" The y coordinate.")
                    .with_access(Access::Public),
            )
            .with_field(Field::from(("z", "int")).with_access(Access::Public));
        assert_eq!(
            c.to_code(),
            "struct Point {\n    bool operator==(const Point& other) const = default;\n    void reset();\n\n    /// Clears the point.\n    void clear();\n\n    void a() {}\n\n    void b() {}\n\n    int x;\n\n    /// The y coordinate.\n    int y;\n\n    int z;\n};\n"
        );
    }
}
