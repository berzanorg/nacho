use ark_ff::BigInteger256;

use crate::{Field, ToFields};

/// The data structure that represents a user's liquidity in a pair of two specific tokens.
#[derive(Clone)]
pub struct Liquidity {
    pub index: u64,
    pub base_token_id: BigInteger256,
    pub quote_token_id: BigInteger256,
    pub points: u128,
}

impl ToFields<4> for Liquidity {
    fn to_fields(&self) -> [Field; 4] {
        [
            self.index.into(),
            self.base_token_id.into(),
            self.quote_token_id.into(),
            self.points.into(),
        ]
    }
}

impl From<&Liquidity> for [u8; 88] {
    fn from(value: &Liquidity) -> Self {
        let mut buf = [0_u8; 88];

        buf[0..8].copy_from_slice(&value.index.to_le_bytes());
        buf[8..16].copy_from_slice(&value.base_token_id.0[0].to_le_bytes());
        buf[16..24].copy_from_slice(&value.base_token_id.0[1].to_le_bytes());
        buf[24..32].copy_from_slice(&value.base_token_id.0[2].to_le_bytes());
        buf[32..40].copy_from_slice(&value.base_token_id.0[3].to_le_bytes());
        buf[40..48].copy_from_slice(&value.quote_token_id.0[0].to_le_bytes());
        buf[48..56].copy_from_slice(&value.quote_token_id.0[1].to_le_bytes());
        buf[56..64].copy_from_slice(&value.quote_token_id.0[2].to_le_bytes());
        buf[64..72].copy_from_slice(&value.quote_token_id.0[3].to_le_bytes());
        buf[72..88].copy_from_slice(&value.points.to_le_bytes());

        buf
    }
}

impl From<[u8; 88]> for Liquidity {
    fn from(value: [u8; 88]) -> Self {
        Self {
            index: u64::from_le_bytes(value[0..8].try_into().unwrap()),
            base_token_id: BigInteger256([
                u64::from_le_bytes(value[8..16].try_into().unwrap()),
                u64::from_le_bytes(value[16..24].try_into().unwrap()),
                u64::from_le_bytes(value[24..32].try_into().unwrap()),
                u64::from_le_bytes(value[32..40].try_into().unwrap()),
            ]),
            quote_token_id: BigInteger256([
                u64::from_le_bytes(value[40..48].try_into().unwrap()),
                u64::from_le_bytes(value[48..56].try_into().unwrap()),
                u64::from_le_bytes(value[56..64].try_into().unwrap()),
                u64::from_le_bytes(value[64..72].try_into().unwrap()),
            ]),
            points: u128::from_le_bytes(value[72..88].try_into().unwrap()),
        }
    }
}
