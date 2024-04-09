use crate::{
    ByteConversion, Field, Scalar, {field_to_scalar, scalar_to_field},
};
use mina_signer::Signature as MinaSignature;

/// A signature that is created by signing a message.
#[derive(Clone, Debug)]
pub struct Signature {
    pub inner: MinaSignature,
}

impl ByteConversion<64> for Signature {
    fn to_bytes(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];

        bytes[0..32].copy_from_slice(&self.inner.rx.to_bytes());
        bytes[32..64].copy_from_slice(&scalar_to_field(self.inner.s).to_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8; 64]) -> Self {
        Signature::new(
            Field::from_bytes(bytes[0..32].try_into().unwrap()),
            field_to_scalar(Field::from_bytes(bytes[32..64].try_into().unwrap())),
        )
    }
}

impl Signature {
    pub fn new(r: Field, s: Scalar) -> Signature {
        Signature {
            inner: MinaSignature::new(r, s),
        }
    }
}
