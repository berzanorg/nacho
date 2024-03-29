use crate::{Field, FromBytes, ToBytes, ToFields, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pool {
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount: u64,
    pub quote_token_amount: u64,
    pub total_liqudity_points: U256,
}

impl ToFields for Pool {
    type Fields = [Field; 5];

    fn to_fields(&self) -> Self::Fields {
        [
            (&self.base_token_id).into(),
            (&self.quote_token_id).into(),
            self.base_token_amount.into(),
            self.quote_token_amount.into(),
            (&self.total_liqudity_points).into(),
        ]
    }
}

impl FromBytes for Pool {
    type Bytes = [u8; 112];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            base_token_id: U256::from_bytes(bytes[0..32].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[32..64].try_into().unwrap()),
            base_token_amount: u64::from_bytes(bytes[64..72].try_into().unwrap()),
            quote_token_amount: u64::from_bytes(bytes[72..80].try_into().unwrap()),
            total_liqudity_points: U256::from_bytes(bytes[80..112].try_into().unwrap()),
        }
    }
}

impl ToBytes for Pool {
    type Bytes = [u8; 112];

    fn to_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 112];

        bytes[0..32].copy_from_slice(&self.base_token_id.to_bytes());
        bytes[32..64].copy_from_slice(&self.quote_token_id.to_bytes());
        bytes[64..72].copy_from_slice(&self.base_token_amount.to_bytes());
        bytes[72..80].copy_from_slice(&self.quote_token_amount.to_bytes());
        bytes[80..112].copy_from_slice(&self.total_liqudity_points.to_bytes());

        bytes
    }
}
