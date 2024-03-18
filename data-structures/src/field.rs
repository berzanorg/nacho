use ark_ff::BigInteger256;
use mina_curves::pasta::Fp;

use crate::ToFields;

/// An object that represents a field element.
///
/// You can think of a field element as a the basic unit of data in zero knowledge proof programming.
///
/// Each field element can store a number up to almost 256 bits in size.
///
/// The maximum value that a field can store is 28,948,022,309,329,048,855,892,746,252,171,976,963,363,056,481,941,560,715,954,676,764,349,967,630,336.
///
/// # Examples
///
/// ```rs
/// let field_from_str = "42".parse().unwrap();
/// let field_from_num: Field = 42.into();
///
/// let bytes = field_to_bytes(&field_from_num);
/// let restored_field = field_from_bytes(&bytes);
///
/// assert_eq!(field_from_num, restored_field);
/// ```
///
pub type Field = Fp;

impl ToFields<1> for Field {
    fn to_fields(&self) -> [Field; 1] {
        [*self]
    }
}

/// Converts a field element to its byte representation.
///
/// # Examples
///
/// ```rs
/// let bytes = field_to_bytes(&field);
/// ```
///
pub fn field_to_bytes(field: &Field) -> [u8; 32] {
    let data = &field.0 .0;

    let mut bytes = [0_u8; 32];

    for i in 0..4 {
        bytes[i * 8..i * 8 + 8].copy_from_slice(&data[i].to_le_bytes());
    }

    bytes
}

/// Constructs a field element from bytes.
///
/// # Examples
///
/// ```rs
/// let field = field_from_bytes(&bytes);
/// ```
///
pub fn field_from_bytes(bytes: &[u8; 32]) -> Field {
    Field::new(BigInteger256::new([
        u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
        u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
        u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
        u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
    ]))
}

#[test]
fn test_field_bytes_conversion() {
    let original_field: Field = "4657651324657865143213749874615453498767487414568798746541"
        .parse()
        .unwrap();

    let bytes = field_to_bytes(&original_field);

    let restored_field = field_from_bytes(&bytes);

    assert_eq!(original_field, restored_field);
}
