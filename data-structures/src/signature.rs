use crate::FromBytes;

/// Byte representation of base 58 encoded signatures.
pub struct Signature([u8; 96]);

impl FromBytes for Signature {
    type Bytes = [u8; 96];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Signature(bytes)
    }
}
