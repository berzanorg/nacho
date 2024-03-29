/// The trait that is implemented for the data structures that can be converted to `Field` elements.
///
/// # Examples
///
/// Define a type to implement `ToFields`:
///
/// ```rs
/// pub struct User {
///     pub age: Field,
///     pub points: Field.
/// }
///```
///
/// Implement `ToFields`:
///
/// ```rs
/// impl ToFields<2> for User {
///     fn to_fields(&self) -> [Field; 2] {
///         [self.age, self.points]
///     }
/// }
/// ```
///
pub trait ToFields {
    type Fields;
    /// Converts the data into `Field` elements.
    ///
    /// # Examples
    ///
    /// Convert a data into fields:
    ///
    /// ```rs
    /// let fields = data.to_fields();
    /// ```
    ///
    fn to_fields(&self) -> Self::Fields;
}
