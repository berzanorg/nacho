use ark_ff::{BigInteger256, PrimeField};
use data_structures::Field;

/// The element that represents a sibling of a leaf or one that leaf's hashes.
#[derive(Clone, Copy)]
pub struct Sibling {
    pub value: Field,
    pub is_left: bool,
}

impl Default for Sibling {
    fn default() -> Self {
        Self {
            value: Field::new(BigInteger256([0; 4])),
            is_left: false,
        }
    }
}

impl From<&Sibling> for [u8; 33] {
    fn from(value: &Sibling) -> Self {
        let mut buf = [0u8; 33];

        let value_value = value.value.into_repr();

        buf[0..8].copy_from_slice(&value_value.0[0].to_le_bytes());
        buf[8..16].copy_from_slice(&value_value.0[1].to_le_bytes());
        buf[16..24].copy_from_slice(&value_value.0[2].to_le_bytes());
        buf[24..32].copy_from_slice(&value_value.0[3].to_le_bytes());
        buf[32] = value.is_left as u8;

        buf
    }
}
