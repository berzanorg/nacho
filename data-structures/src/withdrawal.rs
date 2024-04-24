use crate::{Address, ByteConversion, Field, FieldConversion, U256};

/// The data structure that represents a user's withdrawals of a single token.
///
/// It is stored inside Withdrawals DB.
///
/// - The `withdrawer` property represents the related user's address.
/// - The `token_id` property represents the related token's identifier.
/// - The `token_amount` property represents the total withdrawn token amount of the user.
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Withdrawal {
    pub withdrawer: Address,
    pub token_id: U256,
    pub token_amount: u64,
}

impl FieldConversion<4> for Withdrawal {
    fn to_fields(&self) -> [Field; 4] {
        let [field_0, field_1] = self.withdrawer.to_fields();
        let [field_2] = self.token_id.to_fields();
        let [field_3] = self.token_amount.to_fields();

        [field_0, field_1, field_2, field_3]
    }
}

impl ByteConversion<95> for Withdrawal {
    fn to_bytes(&self) -> [u8; 95] {
        let mut bytes = [0u8; 95];

        bytes[0..55].copy_from_slice(&self.withdrawer.to_bytes());
        bytes[55..87].copy_from_slice(&self.token_id.to_bytes());
        bytes[87..95].copy_from_slice(&self.token_amount.to_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8; 95]) -> Self {
        Self {
            withdrawer: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            token_amount: u64::from_bytes(bytes[87..95].try_into().unwrap()),
        }
    }
}
