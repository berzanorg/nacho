use mina_signer::{create_kimchi, PubKey, Signer as MinaSigner};
use nacho_data_structures::{Field, Signature};

use crate::Message;

/// The signer that verifies Schnorr signatures.
///
/// # Examples
///
///
/// Create a signer:
///
/// ```rs
/// let signer = Signer::new();
/// ```
///
/// Check if a signature is valid:
///
/// ```rs
/// let is_valid = signer.check_signature(&signature, &pubkey, msg);
/// ```
///
///
pub struct Signer {
    inner: Box<dyn MinaSigner<Message>>,
}

impl Signer {
    /// Creates a new `Signer`.
    ///
    /// # Examples
    ///
    /// ```rs
    /// let signer = Signer::new();
    /// ```
    ///
    pub fn new() -> Signer {
        let mina_signer = create_kimchi::<Message>(());
        Signer {
            inner: Box::new(mina_signer),
        }
    }

    /// Checks if the given signature is valid.
    ///
    /// Returns `true` if the signature is valid, otherwise returns `false`.
    ///
    /// # Examples
    ///
    /// Check if a signature is valid:
    /// ```rs
    /// let is_valid = signer.check_signature(&signature, &pubkey, msg);
    /// ```
    ///
    pub fn check_signature(
        &mut self,
        signature: &Signature,
        pubkey: &PubKey,
        msg: Vec<Field>,
    ) -> bool {
        self.inner.verify(&signature.inner, pubkey, &Message(msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_signer() {
        let signer = Signer::new();
        let _ = signer;
    }

    #[test]
    fn checks_signatures() {
        let mut signer = Signer::new();

        let signature = Signature::new(
            "12626512238778647911464193864684212148694812649518775397660142722643855540120"
                .parse()
                .unwrap(),
            "6857963855629205000115548765552901267271269444339596195027304307126718438170"
                .parse()
                .unwrap(),
        );

        let pubkey =
            PubKey::from_address("B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb")
                .unwrap();

        let msg = vec![Field::from(42)];

        let is_valid = signer.check_signature(&signature, &pubkey, msg);

        assert_eq!(is_valid, true);
    }

    #[test]
    fn does_not_validate_signatures_with_mistaken_messages() {
        let mut signer = Signer::new();

        let signature = Signature::new(
            "12626512238778647911464193864684212148694812649518775397660142722643855540120"
                .parse()
                .unwrap(),
            "6857963855629205000115548765552901267271269444339596195027304307126718438170"
                .parse()
                .unwrap(),
        );

        let pubkey =
            PubKey::from_address("B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb")
                .unwrap();

        let msg = vec![Field::from(999999999)];

        let is_valid = signer.check_signature(&signature, &pubkey, msg);

        assert_eq!(is_valid, false);
    }

    #[test]
    fn does_not_validate_signatures_with_mistaken_pubkeys() {
        let mut signer = Signer::new();

        let signature = Signature::new(
            "12626512238778647911464193864684212148694812649518775397660142722643855540120"
                .parse()
                .unwrap(),
            "6857963855629205000115548765552901267271269444339596195027304307126718438170"
                .parse()
                .unwrap(),
        );

        let pubkey =
            PubKey::from_address("B62qk3ncNKNpX7HkMGPE13Y6XczEMcLPpS4ycyRPwUFzKb5XbNEeXSC")
                .unwrap();

        let msg = vec![Field::from(42)];

        let is_valid = signer.check_signature(&signature, &pubkey, msg);

        assert_eq!(is_valid, false);
    }
}
