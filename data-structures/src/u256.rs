use ark_ff::{BigInteger256, PrimeField};

use crate::{Field, FromBytes, ToBytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub [u8; 32]);

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

impl From<&U256> for Field {
    fn from(value: &U256) -> Self {
        let u64s: [u64; 4] = [
            u64::from_le_bytes(value.0[0..8].try_into().unwrap()),
            u64::from_le_bytes(value.0[8..16].try_into().unwrap()),
            u64::from_le_bytes(value.0[16..24].try_into().unwrap()),
            u64::from_le_bytes(value.0[24..32].try_into().unwrap()),
        ];

        let bigint = BigInteger256(u64s);

        let field: Field = bigint.into();

        field
    }
}

impl From<Field> for U256 {
    fn from(value: Field) -> Self {
        let mut buf = [0u8; 32];

        let bigint = value.into_repr();

        let u64s = bigint.0;

        buf[0..8].copy_from_slice(&u64s[0].to_le_bytes());
        buf[8..16].copy_from_slice(&u64s[1].to_le_bytes());
        buf[16..24].copy_from_slice(&u64s[2].to_le_bytes());
        buf[24..32].copy_from_slice(&u64s[3].to_le_bytes());

        let u256 = U256(buf);

        u256
    }
}
