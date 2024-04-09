mod poseidon_hash;

pub use poseidon_hash::{create_poseidon_hasher, poseidon_hash, PoseidonHasher};

#[cfg(test)]
mod tests {

    use mina_hasher::{Hashable, ROInput};
    use mina_signer::{BaseField, PubKey, Signature, Signer};
    use nacho_data_structures::Field;

    #[derive(Clone)]
    pub struct Message {
        pub fields: Vec<BaseField>,
    }

    impl Hashable for Message {
        type D = ();

        fn to_roinput(&self) -> ROInput {
            let mut ro = ROInput::new();

            for field in &self.fields {
                ro = ro.append_field(field.clone())
            }

            ro
        }

        fn domain_string(_: ()) -> Option<String> {
            Some("CodaSignature*******".to_string())
        }
    }

    pub fn verify(signature: &Signature, pubkey: &PubKey, msg: &Message) -> bool {
        let mut signer = mina_signer::create_kimchi::<Message>(());

        let is_verified = signer.verify(signature, pubkey, msg);

        is_verified
    }

    #[test]
    fn test() {
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

        let is_valid = verify(
            &signature,
            &pubkey,
            &Message {
                fields: vec![Field::from(42)],
            },
        );
        assert_eq!(is_valid, true);
    }
}
