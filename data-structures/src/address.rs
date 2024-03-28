use crate::{FromBytes, ToBytes};

/// Byte representation of base 58 encoded public keys.
pub struct Address([u8; 55]);

impl FromBytes for Address {
    type Bytes = [u8; 55];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Address(bytes)
    }
}

impl ToBytes for Address {
    type Bytes = [u8; 55];

    fn to_bytes(&self) -> Self::Bytes {
        self.0
    }
}
