use crate::kotlin::{Function, NestedType, Object, Property, SignatureDec};
use crate::{CodeBuffer, Statement};

/// The members of a class, an interface or an object.
#[derive(Default)]
pub(super) struct Members<'a> {
    pub(super) properties: &'a [Property],
    pub(super) signature_decs: &'a [SignatureDec],
    pub(super) functions: &'a [Function],
    pub(super) companion: Option<&'a Object>,
    pub(super) nested_types: &'a [NestedType],
}

impl Members<'_> {
    //! Write

    /// Writes the curly-bracketed members, after the header of their type at the `level`.
    ///
    /// # Note
    /// - A type with no members has no brackets.
    /// - An empty line splits the members, except properties that are each on one line.
    pub(super) fn write(&self, b: &mut CodeBuffer, level: usize) {
        if self.is_empty() {
            return;
        }
        b.write(" {");
        b.end_line();
        let start: usize = b.len();
        let mut previous: Option<&Property> = None;
        for property in self.properties {
            if previous.is_some_and(|previous| previous.spans_lines() || property.spans_lines()) {
                b.end_line();
            }
            property.write(b, level + 1);
            previous = Some(property);
        }
        for signature_dec in self.signature_decs {
            self.split(b, start);
            signature_dec.write(b, level + 1);
        }
        for function in self.functions {
            self.split(b, start);
            function.write(b, level + 1);
        }
        if let Some(companion) = self.companion {
            self.split(b, start);
            companion.write_companion(b, level + 1);
        }
        for nested_type in self.nested_types {
            self.split(b, start);
            nested_type.write(b, level + 1);
        }
        b.indent(level);
        b.push('}');
    }

    /// Checks if there are no members.
    fn is_empty(&self) -> bool {
        self.properties.is_empty()
            && self.signature_decs.is_empty()
            && self.functions.is_empty()
            && self.companion.is_none()
            && self.nested_types.is_empty()
    }

    /// Writes an empty line if a member was written since the `start` of the members.
    fn split(&self, b: &mut CodeBuffer, start: usize) {
        if b.len() > start {
            b.end_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WithStatements;
    use crate::kotlin::{Class, WithComments, WithFunctions};

    /// Writes the `members` after a header at the indent `level`.
    fn code(members: &Members<'_>, level: usize) -> String {
        let mut b = CodeBuffer::default();
        members.write(&mut b, level);
        b.into()
    }

    #[test]
    fn no_members() {
        assert_eq!(code(&Members::default(), 0), "");
    }

    #[test]
    fn properties_on_one_line_each() {
        let properties = [Property::from(("x", "Int")), Property::from(("y", "Int"))];
        let members = Members {
            properties: &properties,
            ..Members::default()
        };
        assert_eq!(code(&members, 0), " {\n    val x: Int\n    val y: Int\n}");
    }

    #[test]
    fn properties_that_span_lines() {
        let properties = [
            Property::from(("x", "Int")),
            Property::from(("y", "Int")).with_comment(" The y."),
            Property::from(("z", "Int")),
        ];
        let members = Members {
            properties: &properties,
            ..Members::default()
        };
        assert_eq!(
            code(&members, 0),
            " {\n    val x: Int\n\n    /**\n     * The y.\n     */\n    val y: Int\n\n    val z: Int\n}"
        );
    }

    #[test]
    fn functions_without_properties() {
        let functions = [Function::from("f"), Function::from("g")];
        let members = Members {
            functions: &functions,
            ..Members::default()
        };
        assert_eq!(code(&members, 0), " {\n    fun f() {}\n\n    fun g() {}\n}");
    }

    #[test]
    fn all_members_in_order() {
        let properties = [Property::from(("x", "Int"))];
        let signature_decs = [SignatureDec::from("f")];
        let functions = [Function::from("g").with_literal("f()")];
        let companion = Object::default().with_function(Function::from("h"));
        let nested_types = [NestedType::from(Class::from("A"))];
        let members = Members {
            properties: &properties,
            signature_decs: &signature_decs,
            functions: &functions,
            companion: Some(&companion),
            nested_types: &nested_types,
        };
        assert_eq!(
            code(&members, 1),
            " {\n        val x: Int\n\n        fun f()\n\n        fun g() {\n            f()\n        }\n\n        companion object {\n            fun h() {}\n        }\n\n        class A\n    }"
        );
    }
}
