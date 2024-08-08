use crate::rust::mods::validate_mod_name;

/// A path of mod names.
///
/// # Example
/// one/two/three   ->  [one, two, three]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ModPath<'a> {
    path: &'a str,
    file_separator: char,
}

impl<'a> ModPath<'a> {
    //! Validation

    /// Validates the mod path.
    #[cfg(feature = "clerr")]
    pub fn validate_path(path: &str, file_separator: char) -> Result<(), clerr::Report> {
        for mod_name in path.split(file_separator) {
            validate_mod_name(mod_name)?;
        }
        Ok(())
    }

    /// Checks if the mod path is valid.
    #[cfg(feature = "clerr")]
    pub fn is_valid_path(path: &str, file_separator: char) -> bool {
        Self::validate_path(path, file_separator).is_ok() // todo -- remove dependency
    }
}

impl<'a> ModPath<'a> {
    //! Construction

    /// Creates a new mod path.
    #[cfg(feature = "clerr")]
    pub fn new(path: &'a str, file_separator: char) -> Result<Self, clerr::Report> {
        Self::validate_path(path, file_separator)?;

        Ok(unsafe { Self::new_unchecked(path, file_separator) })
    }

    /// Creates a new mod path.
    ///
    /// # Unsafe
    /// The `path` must be valid.
    pub unsafe fn new_unchecked(path: &'a str, file_separator: char) -> Self {
        debug_assert!(Self::is_valid_path(path, file_separator));

        Self {
            path,
            file_separator,
        }
    }
}

impl<'a> ModPath<'a> {
    //! Properties

    /// Gets the mod path.
    pub fn path(&self) -> &'a str {
        self.path
    }

    /// Creates a new iterator for the path segments.
    pub fn iter(&self) -> impl Iterator<Item = &'a str> {
        self.path.split(self.file_separator)
    }
}

impl<'a> AsRef<str> for ModPath<'a> {
    fn as_ref(&self) -> &str {
        self.path
    }
}
