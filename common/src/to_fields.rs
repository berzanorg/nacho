use crate::Field;

/// The trait that is implemented for the data structures that can be converted to `Field` elements.
///
/// # Examples
///
/// ```rs
/// pub struct PublicKey {
///     pub values: (Field, Field),
/// }
///
/// impl ToFields<2> for PublicKey {
///     fn to_fields(&self) -> [Field; 2] {
///         [self.values.0, self.values.1]
///     }
/// }
/// ```
///
pub trait ToFields<const N: usize> {
    /// Converts the data into `Field` elements.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let fields = data.to_fields();
    /// ```
    ///
    fn to_fields(&self) -> [Field; N];
}
