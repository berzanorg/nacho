use crate::{Address, ByteConversion, Field, FieldConversion, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Liquidity {
    pub provider: Address,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub points: U256,
}

impl FieldConversion<5> for Liquidity {
    fn to_fields(&self) -> [Field; 5] {
        let [field_0, field_1] = self.provider.to_fields();
        let [field_2] = self.base_token_id.to_fields();
        let [field_3] = self.quote_token_id.to_fields();
        let [field_4] = self.points.to_fields();

        [field_0, field_1, field_2, field_3, field_4]
    }
}

impl ByteConversion<151> for Liquidity {
    fn to_bytes(&self) -> [u8; 151] {
        let mut bytes = [0u8; 151];

        bytes[0..55].copy_from_slice(&self.provider.to_bytes());
        bytes[55..87].copy_from_slice(&self.base_token_id.to_bytes());
        bytes[87..119].copy_from_slice(&self.quote_token_id.to_bytes());
        bytes[119..151].copy_from_slice(&self.points.to_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8; 151]) -> Self {
        Self {
            provider: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[87..119].try_into().unwrap()),
            points: U256::from_bytes(bytes[119..151].try_into().unwrap()),
        }
    }
}
