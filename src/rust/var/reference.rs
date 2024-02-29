use crate::{CodeBuffer, Expression};

/// A reference.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Reference {
    mutable: bool,
    lifetime: Option<Option<char>>,
}

impl Reference {
    //! Special References

    pub const SHARED: Self = Self {
        mutable: false,
        lifetime: None,
    };

    pub const SHARED_STATIC: Self = Self {
        mutable: false,
        lifetime: Some(None),
    };

    pub const SHARED_A: Self = Self {
        mutable: false,
        lifetime: Some(Some('a')),
    };

    pub const SHARED_B: Self = Self {
        mutable: false,
        lifetime: Some(Some('b')),
    };

    pub const MUT: Self = Self {
        mutable: true,
        lifetime: None,
    };

    pub const MUT_STATIC: Self = Self {
        mutable: true,
        lifetime: Some(None),
    };

    pub const MUT_A: Self = Self {
        mutable: true,
        lifetime: Some(Some('a')),
    };

    pub const MUT_B: Self = Self {
        mutable: true,
        lifetime: Some(Some('b')),
    };
}

impl Reference {
    //! Mutations

    /// Sets the lifetime as mutable.
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

    /// Sets the static lifetime.
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
                b.write(format!("{} ", lifetime).as_str()); // todo -- allocation
            } else {
                b.write("static ")
            }
            if self.mutable {
                b.write("mut ");
            }
        } else {
            if self.mutable {
                b.write("mut ");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rust::Reference;
    use crate::CodeBuffer;

    #[test]
    fn write() {
        let reference: Reference = Reference::SHARED;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&");

        let reference: Reference = Reference::SHARED_STATIC;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'static ");

        let reference: Reference = Reference::SHARED_A;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'a ");

        let reference: Reference = Reference::SHARED_B;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'b ");

        let reference: Reference = Reference::MUT;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&mut ");

        let reference: Reference = Reference::MUT_STATIC;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'static mut ");

        let reference: Reference = Reference::MUT_A;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'a mut ");

        let reference: Reference = Reference::MUT_B;
        let result: String = CodeBuffer::display_expression(&reference);
        assert_eq!(result, "&'b mut ");
    }
}
