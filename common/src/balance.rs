use crate::{Field, PublicKey, ToFields, Uint64};

/// The data structure that represents a user's balance for a specific token.
pub struct Balance {
    pub wdb_index: Field,
    pub token_id: Field,
    pub token_amount: Uint64,
    pub token_owner: PublicKey,
}

impl ToFields<5> for Balance {
    fn to_fields(&self) -> [Field; 5] {
        [
            self.wdb_index,
            self.token_id,
            self.token_amount.value,
            self.token_owner.values.0,
            self.token_owner.values.1,
        ]
    }
}
