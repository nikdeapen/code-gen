use crate::{CodeBuffer, Expression};

/// An access level.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub enum Access {
    #[default]
    Private,
    Public,
    PublicInCrate,
    PublicInSuper,
    PublicInPath(String),
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
    use super::*;

    #[test]
    fn private() {
        assert_eq!(Access::Private.to_code(), "");
    }

    #[test]
    fn public() {
        assert_eq!(Access::Public.to_code(), "pub ");
    }

    #[test]
    fn public_in_crate() {
        assert_eq!(Access::PublicInCrate.to_code(), "pub(crate) ");
    }

    #[test]
    fn public_in_super() {
        assert_eq!(Access::PublicInSuper.to_code(), "pub(super) ");
    }

    #[test]
    fn public_in_path() {
        assert_eq!(
            Access::PublicInPath("crate::foo".into()).to_code(),
            "pub(in crate::foo) "
        );
    }
}
