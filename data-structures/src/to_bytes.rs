pub trait ToBytes {
    type Bytes;

    fn to_bytes(&self) -> Self::Bytes;
}

impl ToBytes for u64 {
    type Bytes = [u8; 8];

    fn to_bytes(&self) -> Self::Bytes {
        self.to_le_bytes()
    }
}

impl ToBytes for u128 {
    type Bytes = [u8; 16];

    fn to_bytes(&self) -> Self::Bytes {
        self.to_le_bytes()
    }
}
