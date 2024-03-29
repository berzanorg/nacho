use crate::{Address, Field, FromBytes, ToBytes, ToFields, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Balance {
    pub owner: Address,
    pub token_id: U256,
    pub token_amount: u64,
}

impl ToFields for Balance {
    type Fields = [Field; 4];

    fn to_fields(&self) -> Self::Fields {
        let owner = self.owner.to_fields();

        [
            owner[0],
            owner[1],
            (&self.token_id).into(),
            self.token_amount.into(),
        ]
    }
}

impl FromBytes for Balance {
    type Bytes = [u8; 95];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            owner: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            token_amount: u64::from_bytes(bytes[87..95].try_into().unwrap()),
        }
    }
}

impl ToBytes for Balance {
    type Bytes = [u8; 95];

    fn to_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 95];

        bytes[0..55].copy_from_slice(&self.owner.to_bytes());
        bytes[55..87].copy_from_slice(&self.token_id.to_bytes());
        bytes[87..95].copy_from_slice(&self.token_amount.to_bytes());

        bytes
    }
}
