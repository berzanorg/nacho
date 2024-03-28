use crate::{FromBytes, ToBytes};

pub struct U256([u8; 32]);

impl FromBytes for U256 {
    type Bytes = [u8; 32];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        U256(bytes)
    }
}

impl ToBytes for U256 {
    type Bytes = [u8; 32];

    fn to_bytes(&self) -> Self::Bytes {
        self.0
    }
}
