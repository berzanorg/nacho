use crate::{Field, ToFields, Uint64};

/// The data structure that represents a user's liquidity in a pair of two specific tokens.
#[derive(Clone, Copy)]
pub struct Liquidity {
    pub wdb_index: Field,
    pub base_token_id: Field,
    pub quote_token_id: Field,
    pub points: Uint64,
}

impl ToFields<4> for Liquidity {
    fn to_fields(&self) -> [Field; 4] {
        [
            self.wdb_index,
            self.base_token_id,
            self.quote_token_id,
            self.points.value,
        ]
    }
}
