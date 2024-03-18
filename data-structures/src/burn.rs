use crate::{Field, ToFields, Uint64};

/// The data structure that represents a user's burn of a specific token.
#[derive(Clone, Copy)]
pub struct Burn {
    pub wdb_index: Field,
    pub token_id: Field,
    pub token_amount: Uint64,
}

impl ToFields<3> for Burn {
    fn to_fields(&self) -> [Field; 3] {
        [self.wdb_index, self.token_id, self.token_amount.value]
    }
}
