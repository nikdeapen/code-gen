use crate::{CodeBuffer, Expression};

/// An access level of a class member.
///
/// # Note
/// - The cases are in the order a class writes its sections.
#[must_use]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    Public,
    Protected,
    #[default]
    Private,
}

impl Expression for Access {
    fn write(&self, b: &mut CodeBuffer) {
        let keyword: &str = match self {
            Self::Public => "public",
            Self::Protected => "protected",
            Self::Private => "private",
        };
        b.write(keyword);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "public");
    }

    #[test]
    fn protected() {
        assert_eq!(Access::Protected.to_code(), "protected");
    }

    #[test]
    fn private() {
        assert_eq!(Access::Private.to_code(), "private");
    }

    #[test]
    fn default_is_private() {
        assert_eq!(Access::default(), Access::Private);
    }
}
