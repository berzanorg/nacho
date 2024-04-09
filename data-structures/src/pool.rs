use crate::{ByteConversion, Field, FieldConversion, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pool {
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount: u64,
    pub quote_token_amount: u64,
    pub total_liqudity_points: U256,
}

impl FieldConversion<5> for Pool {
    fn to_fields(&self) -> [Field; 5] {
        let [field_0] = self.base_token_id.to_fields();
        let [field_1] = self.quote_token_id.to_fields();
        let [field_2] = self.base_token_amount.to_fields();
        let [field_3] = self.quote_token_amount.to_fields();
        let [field_4] = self.total_liqudity_points.to_fields();

        [field_0, field_1, field_2, field_3, field_4]
    }
}

impl ByteConversion<112> for Pool {
    fn to_bytes(&self) -> [u8; 112] {
        let mut bytes = [0u8; 112];

        bytes[0..32].copy_from_slice(&self.base_token_id.to_bytes());
        bytes[32..64].copy_from_slice(&self.quote_token_id.to_bytes());
        bytes[64..72].copy_from_slice(&self.base_token_amount.to_bytes());
        bytes[72..80].copy_from_slice(&self.quote_token_amount.to_bytes());
        bytes[80..112].copy_from_slice(&self.total_liqudity_points.to_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8; 112]) -> Self {
        Self {
            base_token_id: U256::from_bytes(bytes[0..32].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[32..64].try_into().unwrap()),
            base_token_amount: u64::from_bytes(bytes[64..72].try_into().unwrap()),
            quote_token_amount: u64::from_bytes(bytes[72..80].try_into().unwrap()),
            total_liqudity_points: U256::from_bytes(bytes[80..112].try_into().unwrap()),
        }
    }
}
