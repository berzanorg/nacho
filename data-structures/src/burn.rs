use crate::{Address, Field, FromBytes, ToBytes, ToFields, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Burn {
    pub burner: Address,
    pub token_id: U256,
    pub token_amount: u64,
}

impl ToFields for Burn {
    type Fields = [Field; 4];

    fn to_fields(&self) -> Self::Fields {
        let burner = self.burner.to_fields();

        [
            burner[0],
            burner[1],
            (&self.token_id).into(),
            self.token_amount.into(),
        ]
    }
}

impl FromBytes for Burn {
    type Bytes = [u8; 95];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Self {
            burner: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            token_amount: u64::from_bytes(bytes[87..95].try_into().unwrap()),
        }
    }
}

impl ToBytes for Burn {
    type Bytes = [u8; 95];

    fn to_bytes(&self) -> Self::Bytes {
        let mut bytes = [0u8; 95];

        bytes[0..55].copy_from_slice(&self.burner.to_bytes());
        bytes[55..87].copy_from_slice(&self.token_id.to_bytes());
        bytes[87..95].copy_from_slice(&self.token_amount.to_bytes());

        bytes
    }
}
