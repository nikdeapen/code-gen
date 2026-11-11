use crate::{CodeBuffer, Expression};

/// An access level.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    /// No keyword. A member is private & a top-level type is internal.
    #[default]
    Default,
    Public,
    Internal,
    Protected,
    Private,
    ProtectedInternal,
    PrivateProtected,
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        let s: &str = match self {
            Self::Default => "",
            Self::Public => "public ",
            Self::Internal => "internal ",
            Self::Protected => "protected ",
            Self::Private => "private ",
            Self::ProtectedInternal => "protected internal ",
            Self::PrivateProtected => "private protected ",
        };
        b.write(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Access::Default.to_code(), "");
    }

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "public ");
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
    fn protected_internal() {
        assert_eq!(Access::ProtectedInternal.to_code(), "protected internal ");
    }

    #[test]
    fn private_protected() {
        assert_eq!(Access::PrivateProtected.to_code(), "private protected ");
    }
}
