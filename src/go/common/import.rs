use crate::{CodeBuffer, Expression};

/// An imported package. (ex: `"fmt"` or `pp "example.com/protopacket"`)
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Import {
    alias: Option<String>,
    path: String,
}

impl<S: Into<String>> From<S> for Import {
    fn from(path: S) -> Self {
        Self {
            alias: None,
            path: path.into(),
        }
    }
}

impl Import {
    //! Alias

    /// Gets the optional alias.
    #[must_use]
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Sets the alias.
    pub fn set_alias<S>(&mut self, alias: S)
    where
        S: Into<String>,
    {
        self.alias = Some(alias.into());
    }

    /// Sets the alias.
    #[must_use]
    pub fn with_alias<S>(mut self, alias: S) -> Self
    where
        S: Into<String>,
    {
        self.set_alias(alias);
        self
    }
}

impl Import {
    //! Path

    /// Gets the import path.
    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
}

impl Expression for Import {
    fn write(&self, b: &mut CodeBuffer) {
        if let Some(alias) = &self.alias {
            b.write(alias.as_str());
            b.space();
        }
        b.push('"');
        b.write(self.path.as_str());
        b.push('"');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import() {
        let i = Import::from("encoding/binary");
        assert_eq!(i.to_code(), "\"encoding/binary\"");
    }

    #[test]
    fn import_with_alias() {
        let i = Import::from("example.com/protopacket").with_alias("pp");
        assert_eq!(i.to_code(), "pp \"example.com/protopacket\"");
    }
}
