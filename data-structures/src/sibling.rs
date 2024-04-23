use crate::{ByteConversion, U256};

/// The element that represents a sibling of a leaf or one of the hashes of a leaf and its siblings.
///
/// - The `value` property represents the value of a sibling.
/// - The `is_left` property represents the side of a sibling.
#[derive(Clone, Debug)]
pub struct Sibling {
    /// The value of the sibling.
    pub value: U256,
    /// The side of the sibling.
    pub is_left: bool,
}

impl ByteConversion<33> for Sibling {
    fn to_bytes(&self) -> [u8; 33] {
        let mut buf = [0u8; 33];

        buf[0..32].copy_from_slice(&self.value.to_bytes());
        buf[32] = self.is_left as u8;

        buf
    }

    fn from_bytes(_: &[u8; 33]) -> Self {
        panic!("this function is not intended for use")
    }
}

impl Default for Sibling {
    fn default() -> Self {
        Self {
            value: U256([0; 32]),
            is_left: false,
        }
    }
}
