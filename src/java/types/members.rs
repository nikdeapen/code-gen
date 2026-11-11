use crate::java::{Constructor, Field, Function, NestedType, SignatureDec, WithComments};
use crate::{CodeBuffer, IsEmpty, Statement};

/// The members of a type declaration.
///
/// # Note
/// - They are written in the order of the fields here.
/// - A blank line splits two members, unless both are fields or signature decs with no comments.
#[derive(Default)]
pub(super) struct Members {
    pub(super) fields: Vec<Field>,
    pub(super) constructors: Vec<Constructor>,
    pub(super) signature_decs: Vec<SignatureDec>,
    pub(super) functions: Vec<Function>,
    pub(super) nested_types: Vec<NestedType>,
}

impl Members {
    //! Order

    /// Gets the members in the order they are written, each with whether it is always one line.
    fn in_order(&self) -> Vec<(&dyn Statement, bool)> {
        let mut members: Vec<(&dyn Statement, bool)> = Vec::default();
        for field in &self.fields {
            members.push((field, field.comments().is_empty()));
        }
        for constructor in &self.constructors {
            members.push((constructor, false));
        }
        for signature_dec in &self.signature_decs {
            members.push((signature_dec, signature_dec.comments().is_empty()));
        }
        for function in &self.functions {
            members.push((function, false));
        }
        for nested_type in &self.nested_types {
            members.push((nested_type, false));
        }
        members
    }
}

impl IsEmpty for Members {
    fn is_empty(&self) -> bool {
        self.in_order().is_empty()
    }
}

impl Statement for Members {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        let mut last_is_one_line: Option<bool> = None;
        for (member, is_one_line) in self.in_order() {
            if last_is_one_line.is_some_and(|last| !(last && is_one_line)) {
                b.end_line();
            }
            member.write(b, level);
            last_is_one_line = Some(is_one_line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::{Class, JavaPrimitive, Signature, WithResult};

    #[test]
    fn no_members() {
        let members: Members = Members::default();
        assert!(members.is_empty());
        assert_eq!(members.to_code(), "");
    }

    #[test]
    fn one_line_members_have_no_blank_line() {
        let mut members: Members = Members::default();
        members.fields.push(Field::from(("x", "int")));
        members.fields.push(Field::from(("y", "int")));
        assert!(!members.is_empty());
        assert_eq!(members.to_code(), "int x;\nint y;\n");
    }

    #[test]
    fn members_with_comments_have_a_blank_line() {
        let mut members: Members = Members::default();
        members.fields.push(Field::from(("x", "int")));
        members
            .fields
            .push(Field::from(("y", "int")).with_comment(" The y."));
        members.fields.push(Field::from(("z", "int")));
        assert_eq!(
            members.to_code(),
            "int x;\n\n/**\n * The y.\n */\nint y;\n\nint z;\n"
        );
    }

    #[test]
    fn signature_decs_follow_the_same_rule() {
        let x: Signature = Signature::from("x").with_result(JavaPrimitive::Int);
        let y: Signature = Signature::from("y").with_result(JavaPrimitive::Int);
        let z: Signature = Signature::from("z").with_result(JavaPrimitive::Int);
        let mut members: Members = Members::default();
        members.signature_decs.push(SignatureDec::from(x));
        members.signature_decs.push(SignatureDec::from(y));
        members
            .signature_decs
            .push(SignatureDec::from(z).with_comment(" Gets the z."));
        assert_eq!(
            members.to_code(),
            "int x();\nint y();\n\n/**\n * Gets the z.\n */\nint z();\n"
        );
    }

    #[test]
    fn members_are_written_in_order() {
        let mut members: Members = Members::default();
        members.nested_types.push(Class::from("Inner").into());
        members.functions.push(Function::from("reset"));
        members.functions.push(Function::from("clear"));
        members.signature_decs.push(SignatureDec::from("check"));
        members.constructors.push(Constructor::from("Point"));
        members.fields.push(Field::from(("x", "int")));
        let mut b: CodeBuffer = CodeBuffer::default();
        members.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    int x;\n\n    Point() {}\n\n    void check();\n\n    void reset() {}\n\n    void clear() {}\n\n    class Inner {}\n"
        );
    }
}
