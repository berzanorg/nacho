use crate::{Field, ToFields};
use mina_signer::CompressedPubKey;

/// An alias for `CompressedPubKey`.
pub type PublicKey = CompressedPubKey;

impl ToFields<2> for PublicKey {
    fn to_fields(&self) -> [Field; 2] {
        [self.x, self.is_odd.into()]
    }
}

// TODO: Implement an easy to use API for verification of signatures created via o1js.
// Reference: https://github.com/o1-labs/o1js/blob/02f2ffb6a70389fba198efb161c7e04a87534c95/src/lib/signature.ts#L285
