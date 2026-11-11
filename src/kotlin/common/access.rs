use crate::{CodeBuffer, Expression};

/// An access level.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    /// The default access level. It is not written.
    #[default]
    Public,
    Internal,
    Protected,
    Private,
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Public => {}
            Self::Internal => b.write("internal "),
            Self::Protected => b.write("protected "),
            Self::Private => b.write("private "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "");
    }

    #[test]
    fn internal() {
        assert_eq!(Access::Internal.to_code(), "internal ");
    }

    #[test]
    fn protected() {
        assert_eq!(Access::Protected.to_code(), "protected ");
    }

    #[test]
    fn private() {
        assert_eq!(Access::Private.to_code(), "private ");
    }

    #[test]
    fn default_is_public() {
        assert_eq!(Access::default(), Access::Public);
    }
}
