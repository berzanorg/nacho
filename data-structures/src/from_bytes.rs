pub trait FromBytes {
    type Bytes;

    fn from_bytes(bytes: Self::Bytes) -> Self;
}

impl FromBytes for u64 {
    type Bytes = [u8; 8];

    #[inline(always)]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self::from_le_bytes(bytes)
    }
}

impl FromBytes for u128 {
    type Bytes = [u8; 16];

    #[inline(always)]
    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self::from_le_bytes(bytes)
    }
}
