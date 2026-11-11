use crate::cpp::{
    Access, CommentType, CppType, Var, WithAccess, WithComments, WithCppType, WithVar,
};
use crate::{CodeBuffer, Statement, WithName};

/// A field of a class.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Field {
    comments: Vec<String>,
    access: Access,
    var: Var,
    default_value: Option<String>,
}

impl<V: Into<Var>> From<V> for Field {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            var: var.into(),
            default_value: None,
        }
    }
}

impl WithComments for Field {
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

impl WithAccess for Field {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for Field {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithCppType for Field {
    fn cpp_type(&self) -> &CppType {
        self.var.cpp_type()
    }
}

impl WithVar for Field {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Field {
    //! Default Value

    /// Gets the optional default value.
    #[must_use]
    pub fn default_value(&self) -> Option<&str> {
        self.default_value.as_deref()
    }

    /// Sets the default value.
    pub fn set_default_value<S>(&mut self, default_value: S)
    where
        S: Into<String>,
    {
        self.default_value = Some(default_value.into());
    }

    /// Sets the default value.
    #[must_use]
    pub fn with_default_value<S>(mut self, default_value: S) -> Self
    where
        S: Into<String>,
    {
        self.set_default_value(default_value);
        self
    }
}

impl Statement for Field {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        self.write_var(b);
        if let Some(default_value) = &self.default_value {
            b.write(" = ");
            b.write(default_value.as_str());
        }
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_field() {
        let f = Field::from(("x_", "std::uint32_t"));
        assert_eq!(f.name(), "x_");
        assert_eq!(f.cpp_type(), &CppType::from("std::uint32_t"));
        assert_eq!(f.default_value(), None);
        assert_eq!(f.to_code(), "std::uint32_t x_;\n");
    }

    #[test]
    fn field_with_default_value() {
        let f = Field::from(("x_", "std::uint32_t")).with_default_value("0");
        assert_eq!(f.default_value(), Some("0"));
        assert_eq!(f.to_code(), "std::uint32_t x_ = 0;\n");
    }

    #[test]
    fn field_with_comment() {
        let f = Field::from(("x", "std::uint32_t"))
            .with_comment(" The x coordinate.")
            .with_access(Access::Public);
        assert_eq!(f.access(), Access::Public);
        let mut b = CodeBuffer::default();
        f.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// The x coordinate.\n    std::uint32_t x;\n"
        );
    }
}
