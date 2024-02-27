use crate::{CodeBuffer, Expression};

/// An access level.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Access {
    Private,
    Public,
    PublicInCrate,
    PublicInSuper,
    PublicInPath(String),
}

impl Default for Access {
    fn default() -> Self {
        Self::Private
    }
}

impl<S: Into<String>> From<S> for Access {
    fn from(path: S) -> Self {
        Self::PublicInPath(path.into())
    }
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Private => {}
            Self::Public => b.write("pub "),
            Self::PublicInCrate => b.write("pub(crate) "),
            Self::PublicInSuper => b.write("pub(super) "),
            Self::PublicInPath(path) => {
                b.write("pub(in ");
                b.write(path.as_str());
                b.write(") ");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Access;
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let access: Access = Access::Private;
        let result: String = CodeBuffer::display_expression(&access);
        assert_eq!(result, "");

        let access: Access = Access::Public;
        let result: String = CodeBuffer::display_expression(&access);
        assert_eq!(result, "pub ");

        let access: Access = Access::PublicInCrate;
        let result: String = CodeBuffer::display_expression(&access);
        assert_eq!(result, "pub(crate) ");

        let access: Access = Access::PublicInSuper;
        let result: String = CodeBuffer::display_expression(&access);
        assert_eq!(result, "pub(super) ");

        let access: Access = "crate::path".into();
        let result: String = CodeBuffer::display_expression(&access);
        assert_eq!(result, "pub(in crate::path) ");
    }
}
