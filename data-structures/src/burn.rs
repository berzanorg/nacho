use ark_ff::BigInteger256;

use crate::{Field, ToFields};

/// The data structure that represents a user's burn of a specific token.
#[derive(Clone)]
pub struct Burn {
    pub index: u64,
    pub token_id: BigInteger256,
    pub token_amount: u64,
}

impl ToFields<3> for Burn {
    fn to_fields(&self) -> [Field; 3] {
        [
            self.index.into(),
            self.token_id.into(),
            self.token_amount.into(),
        ]
    }
}

impl From<&Burn> for [u8; 48] {
    fn from(value: &Burn) -> Self {
        let mut buf = [0_u8; 48];

        buf[0..8].copy_from_slice(&value.index.to_le_bytes());
        buf[8..16].copy_from_slice(&value.token_id.0[0].to_le_bytes());
        buf[16..24].copy_from_slice(&value.token_id.0[1].to_le_bytes());
        buf[24..32].copy_from_slice(&value.token_id.0[2].to_le_bytes());
        buf[32..40].copy_from_slice(&value.token_id.0[3].to_le_bytes());
        buf[40..48].copy_from_slice(&value.token_amount.to_le_bytes());

        buf
    }
}

impl From<[u8; 48]> for Burn {
    fn from(value: [u8; 48]) -> Self {
        Self {
            index: u64::from_le_bytes(value[0..8].try_into().unwrap()),
            token_id: BigInteger256([
                u64::from_le_bytes(value[8..16].try_into().unwrap()),
                u64::from_le_bytes(value[16..24].try_into().unwrap()),
                u64::from_le_bytes(value[24..32].try_into().unwrap()),
                u64::from_le_bytes(value[32..40].try_into().unwrap()),
            ]),
            token_amount: u64::from_le_bytes(value[40..48].try_into().unwrap()),
        }
    }
}
