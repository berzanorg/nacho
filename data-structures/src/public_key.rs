use crate::{Field, ToFields};
use mina_signer::CompressedPubKey;

/// An alias for `CompressedPubKey`.

pub type PublicKey = CompressedPubKey;

impl ToFields<2> for PublicKey {
    fn to_fields(&self) -> [Field; 2] {
        [self.x, self.is_odd.into()]
    }
}
