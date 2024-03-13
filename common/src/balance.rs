use crate::{Field, PublicKey, Uint64};

/// The data structure that represents a user's balance for a specific token.
pub struct Balance {
    pub wdb_index: Field,
    pub token_id: Field,
    pub token_amount: Uint64,
    pub token_owner: PublicKey,
}
