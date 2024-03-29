use mina_signer::CompressedPubKey;

use crate::{Field, FromBytes, ToBytes, ToFields};

/// Byte representation of base 58 encoded public keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address([u8; 55]);

impl Address {
    // Example: B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb
    // Regexp: /^B62q[1-9A-HJ-NP-Za-km-z]{51}$/
    pub fn is_valid(addr: &str) -> Result<(), ()> {
        if addr.len() != 55 {
            return Err(());
        }

        if addr[0..4].as_bytes() != [66, 54, 50, 113] {
            return Err(());
        }

        for c in addr.bytes() {
            match c {
                b'1'..=b'9'
                | b'A'..=b'H'
                | b'J'..=b'N'
                | b'P'..=b'Z'
                | b'a'..=b'k'
                | b'm'..=b'z' => (),
                _ => return Err(()),
            }
        }

        Ok(())
    }
}

impl FromBytes for Address {
    type Bytes = [u8; 55];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Address(bytes)
    }
}

impl ToBytes for Address {
    type Bytes = [u8; 55];

    fn to_bytes(&self) -> Self::Bytes {
        self.0
    }
}

impl ToFields for Address {
    type Fields = [Field; 2];

    fn to_fields(&self) -> Self::Fields {
        let pubkey = CompressedPubKey::from_address(std::str::from_utf8(&self.0).unwrap()).unwrap();

        [pubkey.x, pubkey.is_odd.into()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn checks_if_address_is_valid() {
        // /^B62q[1-9A-HJ-NP-Za-km-z]{51}$/

        let addr = "B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), Ok(()));

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), Err(()));

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2ivhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), Err(()));

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2fjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), Err(()));

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2ivhMkU4FfvhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), Err(()));
    }
}
