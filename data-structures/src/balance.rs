use ark_ff::{BigInteger256, PrimeField};

use crate::{Field, PublicKey, ToFields};

/// The data structure that represents a user's balance for a specific token.
#[derive(Clone)]
pub struct Balance {
    pub index: u64,
    pub token_id: BigInteger256,
    pub token_amount: u64,
    pub token_owner: PublicKey,
}

impl ToFields<5> for Balance {
    fn to_fields(&self) -> [Field; 5] {
        [
            self.index.into(),
            self.token_id.into(),
            self.token_amount.into(),
            self.token_owner.x,
            self.token_owner.is_odd.into(),
        ]
    }
}

impl From<[u8; 81]> for Balance {
    fn from(value: [u8; 81]) -> Self {
        Self {
            index: u64::from_le_bytes(value[0..8].try_into().unwrap()),
            token_id: BigInteger256([
                u64::from_le_bytes(value[8..16].try_into().unwrap()),
                u64::from_le_bytes(value[16..24].try_into().unwrap()),
                u64::from_le_bytes(value[24..32].try_into().unwrap()),
                u64::from_le_bytes(value[32..40].try_into().unwrap()),
            ]),
            token_amount: u64::from_le_bytes(value[40..48].try_into().unwrap()),
            token_owner: PublicKey {
                x: BigInteger256([
                    u64::from_le_bytes(value[48..56].try_into().unwrap()),
                    u64::from_le_bytes(value[56..64].try_into().unwrap()),
                    u64::from_le_bytes(value[64..72].try_into().unwrap()),
                    u64::from_le_bytes(value[72..80].try_into().unwrap()),
                ])
                .into(),
                is_odd: value[80] != 0,
            },
        }
    }
}

impl From<Balance> for [u8; 81] {
    fn from(value: Balance) -> Self {
        let mut buf = [0_u8; 81];

        let value_token_owner_x = value.token_owner.x.into_repr();

        buf[0..8].copy_from_slice(&value.index.to_le_bytes());
        buf[8..16].copy_from_slice(&value.token_id.0[0].to_le_bytes());
        buf[16..24].copy_from_slice(&value.token_id.0[1].to_le_bytes());
        buf[24..32].copy_from_slice(&value.token_id.0[2].to_le_bytes());
        buf[32..40].copy_from_slice(&value.token_id.0[3].to_le_bytes());
        buf[40..48].copy_from_slice(&value.token_amount.to_le_bytes());
        buf[48..56].copy_from_slice(&value_token_owner_x.0[0].to_le_bytes());
        buf[56..64].copy_from_slice(&value_token_owner_x.0[1].to_le_bytes());
        buf[64..72].copy_from_slice(&value_token_owner_x.0[2].to_le_bytes());
        buf[72..80].copy_from_slice(&value_token_owner_x.0[3].to_le_bytes());
        buf[80] = value.token_owner.is_odd as u8;

        buf
    }
}
