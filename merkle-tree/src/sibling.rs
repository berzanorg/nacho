use nacho_data_structures::{ToBytes, U256};

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

impl ToBytes for Sibling {
    type Bytes = [u8; 33];

    fn to_bytes(&self) -> Self::Bytes {
        let mut buf = [0u8; 33];

        buf[0..32].copy_from_slice(&self.value.to_bytes());
        buf[32] = self.is_left as u8;

        buf
    }
}
