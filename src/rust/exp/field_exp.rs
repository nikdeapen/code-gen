use crate::{CodeBuffer, Expression, WithName};

/// A struct field expression. `self.field_name`
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct FieldExp {
    field_name: String,
}

impl<S: Into<String>> From<S> for FieldExp {
    fn from(field_name: S) -> Self {
        Self {
            field_name: field_name.into(),
        }
    }
}

impl WithName for FieldExp {
    fn name(&self) -> &str {
        self.field_name.as_str()
    }
}

impl Expression for FieldExp {
    fn write(&self, b: &mut CodeBuffer) {
        b.write("self.");
        self.write_name(b);
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::FieldExp;
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let exp: FieldExp = "field_name".into();
        let result: String = CodeBuffer::display_expression(&exp);
        assert_eq!(result, "self.field_name");
    }
}
