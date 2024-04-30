/// The trait that is used to convert statically sized data structures back and forth into bytes.
///
/// The constant generic parameter `L` represents the length of the byte representation of the data structures.
///
/// # Examples:
///
/// Implement `ByteConversion`:
///
/// ```rs
/// impl ByteConversion<8> for u64 {
///     fn to_bytes(&self) -> [u8; 8] {
///         self.to_le_bytes()
///     }
///
///     fn from_bytes(bytes: &[u8; 8]) -> Self {
///         u64::from_le_bytes(bytes.to_owned())
///     }
/// }
/// ```
///
/// Parse from bytes:
///
/// ```rs
/// let data = u64::from_bytes(&bytes);
/// ```
///
/// Convert into bytes:
///
/// ```rs
/// let bytes = data.to_bytes();
/// ```
///
pub trait ByteConversion<const L: usize> {
    /// Converts the data structure to bytes.
    ///
    /// # Examples
    ///
    /// Convert to bytes:
    ///
    /// ```rs
    /// let bytes = data.to_bytes();
    /// ```
    ///  
    fn to_bytes(&self) -> [u8; L];

    /// Creates a data structure from the given bytes.
    ///
    /// # Examples
    ///
    /// Create a data structure:
    ///
    /// ```rs
    /// let data = ByteConversion.from_bytes(bytes);
    /// ```
    ///  
    fn from_bytes(bytes: &[u8; L]) -> Self;
}

impl ByteConversion<8> for u64 {
    fn to_bytes(&self) -> [u8; 8] {
        self.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8; 8]) -> Self {
        u64::from_le_bytes(bytes.to_owned())
    }
}

impl ByteConversion<4> for u32 {
    fn to_bytes(&self) -> [u8; 4] {
        self.to_le_bytes()
    }

    fn from_bytes(bytes: &[u8; 4]) -> Self {
        u32::from_le_bytes(bytes.to_owned())
    }
}

impl ByteConversion<1> for bool {
    fn to_bytes(&self) -> [u8; 1] {
        (self.to_owned() as u8).to_le_bytes()
    }

    fn from_bytes(bytes: &[u8; 1]) -> Self {
        bytes[0] != 0
    }
}
