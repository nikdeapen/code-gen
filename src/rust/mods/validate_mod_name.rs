#[cfg(feature = "clerr")]
use clerr::{Code, CodedHeader, PrimaryEntry, Report, Severity};

/// Validates the mod name.
#[cfg(feature = "clerr")]
pub fn validate_mod_name(mod_name: &str) -> Result<(), Report> {
    // todo -- better validation
    if mod_name.is_empty() {
        Err(Report::new(PrimaryEntry::new(
            CodedHeader::new(
                Code::new(Severity::Error, "EMPTY_RUST_MOD"),
                "empty rust module name",
            ),
            vec![],
        )))
    } else if mod_name
        .as_bytes()
        .iter()
        .any(|c| !c.is_ascii_alphanumeric() || *c != b'_')
    {
        Err(Report::new(PrimaryEntry::new(
            CodedHeader::new(
                Code::new(Severity::Error, "INV_RUST_MOD"),
                "invalid rust module name",
            ),
            vec![],
        )))
    } else {
        Ok(())
    }
}

/// Validates the mod name.
#[cfg(feature = "clerr")]
pub fn is_valid_mod_name(mod_name: &str) -> bool {
    validate_mod_name(mod_name).is_ok() // todo -- remove dependency & allocation
}
