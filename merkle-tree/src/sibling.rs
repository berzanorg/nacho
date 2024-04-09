use nacho_data_structures::{ByteConversion, U256};

/// The element that represents a sibling of a leaf or one that leaf's hashes.
#[derive(Clone, Debug)]
pub struct Sibling {
    pub value: U256,
    pub is_left: bool,
}

impl Default for Sibling {
    fn default() -> Self {
        Self {
            value: U256([0; 32]),
            is_left: false,
        }
    }
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
