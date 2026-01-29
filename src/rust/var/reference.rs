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
    pub fn with_mut(self) -> Self {
        Self {
            mutable: true,
            lifetime: self.lifetime,
        }
    }

    /// Sets the lifetime.
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
    pub fn with_static_lifetime(self) -> Self {
        Self {
            mutable: self.mutable,
            lifetime: Some(None),
        }
    }
}

impl Expression for Reference {
    fn write(&self, b: &mut CodeBuffer) {
        b.write("&");
        if let Some(lifetime) = self.lifetime {
            b.write("'");
            if let Some(lifetime) = lifetime {
                let mut buffer: [u8; 4] = [0u8; 4];
                let char_str: &str = lifetime.encode_utf8(&mut buffer);
                b.write(char_str);
                b.write(" ");
            } else {
                b.write("static ")
            }
        }
        if self.mutable {
            b.write("mut ");
        }
    }
}
