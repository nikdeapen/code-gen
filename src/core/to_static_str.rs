/// An element that can convert itself to a static string.
pub trait ToStaticStr {
    /// Converts the element to a static string.
    fn to_static_str(&self) -> &'static str;
}
