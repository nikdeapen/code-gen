use crate::go::{GoType, Var, WithComments, WithGoType, WithVar};
use crate::{CodeBuffer, Statement, WithName};

/// A field of a struct declaration.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct StructField {
    comments: Vec<String>,
    var: Var,
    tag: Option<String>,
}

impl<V: Into<Var>> From<V> for StructField {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            var: var.into(),
            tag: None,
        }
    }
}

impl WithComments for StructField {
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

impl WithName for StructField {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithGoType for StructField {
    fn go_type(&self) -> &GoType {
        self.var.go_type()
    }
}

impl WithVar for StructField {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl StructField {
    //! Tag

    /// Gets the optional tag.
    #[must_use]
    pub fn tag(&self) -> Option<&str> {
        self.tag.as_deref()
    }

    /// Sets the tag.
    ///
    /// # Note
    /// - The tag is written between backticks. (ex: `json:"x"`)
    pub fn set_tag<S>(&mut self, tag: S)
    where
        S: Into<String>,
    {
        self.tag = Some(tag.into());
    }

    /// Sets the tag.
    #[must_use]
    pub fn with_tag<S>(mut self, tag: S) -> Self
    where
        S: Into<String>,
    {
        self.set_tag(tag);
        self
    }
}

impl Statement for StructField {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(b, level);
        b.indent(level);
        self.write_var(b);
        if let Some(tag) = &self.tag {
            b.write(" `");
            b.write(tag.as_str());
            b.push('`');
        }
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_field() {
        let f = StructField::from(("x", "uint32"));
        assert_eq!(f.to_code(), "x uint32\n");
    }

    #[test]
    fn struct_field_with_tag() {
        let f = StructField::from(("Name", GoType::from("string").to_pointer()))
            .with_tag("json:\"name,omitempty\"");
        assert_eq!(f.to_code(), "Name *string `json:\"name,omitempty\"`\n");
    }

    #[test]
    fn struct_field_with_comment() {
        let f = StructField::from(("x", "uint32")).with_comment(" The x coordinate.");
        assert_eq!(f.to_code(), "// The x coordinate.\nx uint32\n");
    }
}
