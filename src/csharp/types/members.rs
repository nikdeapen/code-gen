use crate::csharp::{
    Constructor, Field, Function, Property, SignatureDec, WithAttributes, WithComments,
};
use crate::{CodeBuffer, IsEmpty, Statement};

/// The members of a type declaration.
///
/// # Note
/// - The members are written by kind, in the order of the fields here.
/// - A blank line is between two members, unless they are of one kind & each is one line.
#[derive(Default)]
pub(super) struct Members {
    pub(super) fields: Vec<Field>,
    pub(super) constructors: Vec<Constructor>,
    pub(super) properties: Vec<Property>,
    pub(super) signature_decs: Vec<SignatureDec>,
    pub(super) functions: Vec<Function>,
    pub(super) nested_types: Vec<Box<dyn Statement>>,
}

impl Members {
    //! Write

    /// Writes the `members` of one kind, with the blank lines before them.
    ///
    /// # Note
    /// - `is_first` is true until a member of any kind is written.
    fn write_kind<M>(
        &self,
        b: &mut CodeBuffer,
        level: usize,
        is_first: &mut bool,
        members: &[M],
        is_line: fn(&M) -> bool,
    ) where
        M: Statement,
    {
        let mut last_is_line: bool = false;
        for member in members {
            let this_is_line: bool = is_line(member);
            if !*is_first && !(last_is_line && this_is_line) {
                b.end_line();
            }
            member.write(b, level);
            *is_first = false;
            last_is_line = this_is_line;
        }
    }
}

impl IsEmpty for Members {
    fn is_empty(&self) -> bool {
        self.fields.is_empty()
            && self.constructors.is_empty()
            && self.properties.is_empty()
            && self.signature_decs.is_empty()
            && self.functions.is_empty()
            && self.nested_types.is_empty()
    }
}

impl Statement for Members {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        b.line(level, "{");
        let inner: usize = level + 1;
        let mut is_first: bool = true;
        self.write_kind(b, inner, &mut is_first, &self.fields, |field| {
            field.comments().is_empty()
        });
        self.write_kind(b, inner, &mut is_first, &self.constructors, |_| false);
        self.write_kind(b, inner, &mut is_first, &self.properties, |property| {
            property.comments().is_empty() && property.attributes().is_empty()
        });
        self.write_kind(
            b,
            inner,
            &mut is_first,
            &self.signature_decs,
            |signature_dec| signature_dec.comments().is_empty(),
        );
        self.write_kind(b, inner, &mut is_first, &self.functions, |_| false);
        for nested_type in &self.nested_types {
            if !is_first {
                b.end_line();
            }
            nested_type.write(b, inner);
            is_first = false;
        }
        b.line(level, "}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csharp::Enum;

    #[test]
    fn no_members() {
        let members: Members = Members::default();
        assert!(members.is_empty());
        assert_eq!(members.to_code(), "{\n}\n");
    }

    #[test]
    fn members_of_one_line() {
        let mut members: Members = Members::default();
        members.fields.push(Field::from(("_x", "int")));
        members.fields.push(Field::from(("_y", "int")));
        members.properties.push(Property::from(("X", "int")));
        members.properties.push(Property::from(("Y", "int")));
        members.signature_decs.push(SignatureDec::from("Run"));
        members.signature_decs.push(SignatureDec::from("Stop"));
        assert!(!members.is_empty());
        assert_eq!(
            members.to_code(),
            "\
{
    int _x;
    int _y;

    int X { get; }
    int Y { get; }

    void Run();
    void Stop();
}
"
        );
    }

    #[test]
    fn members_of_many_lines() {
        let mut members: Members = Members::default();
        members.fields.push(Field::from(("_x", "int")));
        members
            .fields
            .push(Field::from(("_y", "int")).with_comment(" The y."));
        members.fields.push(Field::from(("_z", "int")));
        members.properties.push(Property::from(("X", "int")));
        members
            .properties
            .push(Property::from(("Y", "int")).with_attribute("Obsolete"));
        members
            .signature_decs
            .push(SignatureDec::from("Run").with_comment(" Runs the task."));
        members.signature_decs.push(SignatureDec::from("Stop"));
        assert_eq!(
            members.to_code(),
            "\
{
    int _x;

    /// <summary>
    /// The y.
    /// </summary>
    int _y;

    int _z;

    int X { get; }

    [Obsolete]
    int Y { get; }

    /// <summary>
    /// Runs the task.
    /// </summary>
    void Run();

    void Stop();
}
"
        );
    }

    #[test]
    fn members_of_each_kind() {
        let mut members: Members = Members::default();
        members.nested_types.push(Box::new(Enum::from("Kind")));
        members.nested_types.push(Box::new(Enum::from("Form")));
        members.functions.push(Function::from("Run"));
        members.functions.push(Function::from("Stop"));
        members.signature_decs.push(SignatureDec::from("Reset"));
        members.properties.push(Property::from(("X", "int")));
        members.constructors.push(Constructor::from("Point"));
        members.fields.push(Field::from(("_x", "int")));
        let mut b: CodeBuffer = CodeBuffer::default();
        members.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    {\n        int _x;\n\n        Point()\n        {\n        }\n\n        int X { get; }\n\n        void Reset();\n\n        void Run()\n        {\n        }\n\n        void Stop()\n        {\n        }\n\n        enum Kind\n        {\n        }\n\n        enum Form\n        {\n        }\n    }\n"
        );
    }

    #[test]
    fn nested_types_only() {
        let mut members: Members = Members::default();
        members.nested_types.push(Box::new(Enum::from("Kind")));
        assert!(!members.is_empty());
        assert_eq!(members.to_code(), "{\n    enum Kind\n    {\n    }\n}\n");
    }
}
