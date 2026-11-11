use crate::{CodeBuffer, Expression};

/// An access level.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    Public,
    Protected,

    /// The access of a declaration with no keyword.
    #[default]
    Package,

    Private,
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Public => b.write("public "),
            Self::Protected => b.write("protected "),
            Self::Package => {}
            Self::Private => b.write("private "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "public ");
    }

    #[test]
    fn protected() {
        assert_eq!(Access::Protected.to_code(), "protected ");
    }

    #[test]
    fn package() {
        assert_eq!(Access::Package.to_code(), "");
    }

    #[test]
    fn private() {
        assert_eq!(Access::Private.to_code(), "private ");
    }

    #[test]
    fn default_is_package() {
        assert_eq!(Access::default(), Access::Package);
    }
}
