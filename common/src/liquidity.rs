use crate::{Field, Uint64};

/// The data structure that represents a user's liquidity in a pair of two specific tokens.
pub struct Liquidity {
    pub wdb_index: Field,
    pub base_token_id: Field,
    pub quote_token_id: Field,
    pub points: Uint64,
}
