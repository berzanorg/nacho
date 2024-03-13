use crate::{Field, ToFields, Uint64};

/// The data structure that represents a pair of two specific tokens.
pub struct Pair {
    pub wdb_index: Field,
    pub base_token_id: Field,
    pub quote_token_id: Field,
    pub base_token_amount: Uint64,
    pub quote_token_amount: Uint64,
}

impl ToFields<5> for Pair {
    fn to_fields(&self) -> [Field; 5] {
        [
            self.wdb_index,
            self.base_token_id,
            self.quote_token_id,
            self.base_token_amount.value,
            self.quote_token_amount.value,
        ]
    }
}
