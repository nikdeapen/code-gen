use crate::{CodeBuffer, Expression};

/// A reference to a type.
///
/// # Default
/// The default reference is `&`; a shared reference with no lifetime.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Reference {
    mutable: bool,
    lifetime: Option<Option<char>>,
}

impl Reference {
    //! Mutations

    /// Sets the reference to mutable.
    #[must_use]
    pub fn with_mut(self) -> Self {
        Self {
            mutable: true,
            lifetime: self.lifetime,
        }
    }

    /// Sets the lifetime.
    #[must_use]
    pub fn with_lifetime(self, c: char) -> Option<Self> {
        if !c.is_ascii_lowercase() {
            None
        } else {
            Some(Self {
                mutable: self.mutable,
                lifetime: Some(Some(c)),
            })
        }
    }

    /// Sets the lifetime to `static`.
    #[must_use]
    pub fn with_static_lifetime(self) -> Self {
        Self {
            mutable: self.mutable,
            lifetime: Some(None),
        }
    }
}

impl Expression for Reference {
    fn write(&self, b: &mut CodeBuffer) {
        b.push('&');
        if let Some(lifetime) = self.lifetime {
            b.push('\'');
            if let Some(lifetime) = lifetime {
                b.push(lifetime);
                b.push(' ');
            } else {
                b.write("static ")
            }
        }
        if self.mutable {
            b.write("mut ");
        }
    }
}
