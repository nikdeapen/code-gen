use crate::{CodeBuffer, Expression};

/// An access level of a class member.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    /// No keyword, which is public.
    #[default]
    Default,
    Public,
    Protected,
    Private,
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        match self {
            Self::Default => {}
            Self::Public => b.write("public "),
            Self::Protected => b.write("protected "),
            Self::Private => b.write("private "),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Access::Default.to_code(), "");
        assert_eq!(Access::default(), Access::Default);
    }

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "public ");
    }

    #[test]
    fn protected() {
        assert_eq!(Access::Protected.to_code(), "protected ");
    }

    #[test]
    fn private() {
        assert_eq!(Access::Private.to_code(), "private ");
    }
}
