use crate::cpp::{
    Access, CommentType, CppType, Var, WithAccess, WithComments, WithCppType, WithVar,
};
use crate::{CodeBuffer, Statement, WithName};

/// A type alias declaration. (ex: `using Id = std::uint64_t;`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct TypeDec {
    comments: Vec<String>,
    access: Access,
    var: Var,
}

impl<V: Into<Var>> From<V> for TypeDec {
    fn from(var: V) -> Self {
        Self {
            comments: Vec::default(),
            access: Access::default(),
            var: var.into(),
        }
    }
}

impl WithComments for TypeDec {
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

impl WithAccess for TypeDec {
    fn access(&self) -> Access {
        self.access
    }

    fn set_access(&mut self, access: Access) {
        self.access = access;
    }
}

impl WithName for TypeDec {
    fn name(&self) -> &str {
        self.var.name()
    }
}

impl WithCppType for TypeDec {
    fn cpp_type(&self) -> &CppType {
        self.var.cpp_type()
    }
}

impl WithVar for TypeDec {
    fn var(&self) -> &Var {
        &self.var
    }
}

impl Statement for TypeDec {
    fn write(&self, b: &mut CodeBuffer, level: usize) {
        self.write_comments(CommentType::LineDoc, b, level);
        b.indent(level);
        b.write("using ");
        self.write_name(b);
        b.write(" = ");
        self.write_cpp_type(b);
        b.push(';');
        b.end_line();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_type_dec() {
        let t = TypeDec::from(("Id", "std::uint64_t"));
        assert_eq!(t.to_code(), "using Id = std::uint64_t;\n");
    }

    #[test]
    fn type_dec_with_comment() {
        let value = CppType::from("std::variant")
            .with_generic("Circle")
            .with_generic("Square");
        let t = TypeDec::from(Var::from(("Value", value)))
            .with_comment(" The value of a case.")
            .with_access(Access::Public);
        assert_eq!(t.access(), Access::Public);
        let mut b = CodeBuffer::default();
        t.write(&mut b, 1);
        assert_eq!(
            b.peek(),
            "    /// The value of a case.\n    using Value = std::variant<Circle, Square>;\n"
        );
    }
}
