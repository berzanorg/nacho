use crate::Field;

/// A wrapper around two `Field` values to act like a public key.
pub struct PublicKey {
    values: (Field, Field),
}

impl PublicKey {
    /// Creates a new `PublicKey` using the two given `Field` values.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let values: (Field, Field) = (42.into(), 7.into());
    ///
    /// let public_key = PublicKey::new(values);
    /// ```
    ///
    pub fn new(values: (Field, Field)) -> PublicKey {
        PublicKey { values }
    }
}
