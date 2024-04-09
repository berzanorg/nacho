use crate::{ByteConversion, FieldConversion};
use ark_ff::{BigInteger256, PrimeField};
use mina_curves::pasta::Fp;

/// An alias that represents a field element.
///
/// You can think of a field element as a the basic unit of data in zero knowledge proof programming.
///
/// Each field element can store a number up to almost 256 bits in size.
///
/// The maximum value that a field can store is 28,948,022,309,329,048,855,892,746,252,171,976,963,363,056,481,941,560,715,954,676,764,349,967,630,336.
///
/// # Examples
///
/// Create a field from a string:
///
/// ```rs
/// let field = "42".parse().unwrap();
/// ```
///
/// Create a field from a number:
///
/// ```rs
/// let field: Field = 42.into();
/// ```
///
/// Create a field from a `BigInteger256`:
///
/// ```rs
/// let field: Field = big_integer.into();
/// ```
///
/// Creat a field from bytes:
///
/// ```rs
/// let field: Field = Field::from_bytes(&bytes);
/// ```
///
/// Convert a field to bytes:
///
/// ```rs
/// let bytes = field.to_bytes();
/// ```
///
pub type Field = Fp;

impl FieldConversion<1> for Field {
    fn to_fields(&self) -> [Field; 1] {
        [self.to_owned()]
    }
}

impl ByteConversion<32> for Field {
    fn to_bytes(&self) -> [u8; 32] {
        let mut buf = [0u8; 32];

        let u64s = self.into_repr().0;

        buf[0..8].copy_from_slice(&u64s[0].to_bytes());
        buf[8..16].copy_from_slice(&u64s[1].to_bytes());
        buf[16..24].copy_from_slice(&u64s[2].to_bytes());
        buf[24..32].copy_from_slice(&u64s[3].to_bytes());

        buf
    }

    fn from_bytes(bytes: &[u8; 32]) -> Self {
        BigInteger256::new([
            u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
            u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
            u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
            u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
        ])
        .into()
    }
}
