use crate::Field;

/// The trait that is used to convert statically sized data structures to field elements.
///
/// The constant generic parameter `L` represents the length of the field representation of the data structures.
///
/// # Examples:
///
/// Implement `FieldConversion`:
///
/// ```rs
/// impl FieldConversion<1> for u64 {
///     fn to_fields(&self) -> [Field; 1] {
///         let field_0 = self.to_owned().into();
///
///         [field_0]
///     }
/// }
/// ```
///
/// Convert into fields:
///
/// ```rs
/// let fields = data.to_fields();
/// ```
///
pub trait FieldConversion<const L: usize> {
    /// Converts the data structure to field elements.
    ///
    /// # Examples
    ///
    /// Convert to fields:
    ///
    /// ```rs
    /// let fields = data.to_fields();
    /// ```
    ///  
    fn to_fields(&self) -> [Field; L];
}

impl FieldConversion<1> for u64 {
    fn to_fields(&self) -> [Field; 1] {
        let field_0 = self.to_owned().into();

        [field_0]
    }
}

impl FieldConversion<1> for bool {
    fn to_fields(&self) -> [Field; 1] {
        let field_0 = self.to_owned().into();

        [field_0]
    }
}
