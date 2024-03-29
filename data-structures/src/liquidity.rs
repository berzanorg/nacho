use crate::{Address, Field, FromBytes, ToBytes, ToFields, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Liquidity {
    pub provider: Address,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub points: U256,
}

impl ToFields for Liquidity {
    type Fields = [Field; 5];

    fn to_fields(&self) -> Self::Fields {
        let provider = self.provider.to_fields();

        [
            provider[0],
            provider[1],
            (&self.base_token_id).into(),
            (&self.quote_token_id).into(),
            (&self.points).into(),
        ]
    }
}

impl FromBytes for Liquidity {
    type Bytes = [u8; 151];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            provider: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[87..119].try_into().unwrap()),
            points: U256::from_bytes(bytes[119..151].try_into().unwrap()),
        }
    }
}

impl ToBytes for Liquidity {
    type Bytes = [u8; 151];

    fn to_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 151];

        bytes[0..55].copy_from_slice(&self.provider.to_bytes());
        bytes[55..87].copy_from_slice(&self.base_token_id.to_bytes());
        bytes[87..119].copy_from_slice(&self.quote_token_id.to_bytes());
        bytes[119..151].copy_from_slice(&self.points.to_bytes());

        bytes
    }
}
