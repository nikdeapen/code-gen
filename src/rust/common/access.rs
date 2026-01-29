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
