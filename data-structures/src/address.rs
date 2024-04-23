use crate::{ByteConversion, Field, FieldConversion};
use mina_signer::{CompressedPubKey, PubKey};

/// Byte representation of base 58 encoded public keys.
///
/// For example: `B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb`
///
/// You must always call `Address::is_valid` function, then construct an `Address`.
///
/// Otherwise the address might not be a valid public key.
///
/// # Examples:
///
/// Create an address:
///
/// ```rs
/// if Address::is_valid(bytes) {
///     let address = Address(bytes);
///     // Continue as how you want...
/// } else {
///     // Handle error...
/// }
/// ```
///
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Address([u8; 55]);

impl Address {
    /// Converts the address into a `mina_signer::Pubkey`.
    ///
    /// If you use `Address::is_valid` function before calling this function, it is never gonna fail.
    ///
    /// # Examples:
    ///
    /// Convert an address to a public key:
    ///
    /// ```rs
    /// let pubkey = address.to_pubkey().unwrap();
    /// ```
    ///
    pub fn to_pubkey(&self) -> Result<PubKey, ()> {
        PubKey::from_address(std::str::from_utf8(&self.0).map_err(|_| ())?).map_err(|_| ())
    }

    /// Checks the given address.
    ///
    /// Returns `true` if it is a valid address, otherwise returns `false`.
    ///
    /// Example valid address: `B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb`
    ///
    /// Regular expression: `/^B62q[1-9A-HJ-NP-Za-km-z]{51}$/`
    ///
    /// # Examples
    ///
    /// Check an address:
    ///
    /// ```rs
    /// let is_valid = Address::is_valid("B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb");
    /// assert_eq!(is_valid, true);
    /// ```
    ///
    pub fn is_valid(addr: &str) -> bool {
        if addr.len() != 55 {
            return false;
        }

        if addr[0..4].as_bytes() != [66, 54, 50, 113] {
            return false;
        }

        for c in addr.bytes() {
            match c {
                b'1'..=b'9'
                | b'A'..=b'H'
                | b'J'..=b'N'
                | b'P'..=b'Z'
                | b'a'..=b'k'
                | b'm'..=b'z' => (),
                _ => return false,
            }
        }

        true
    }
}

impl FieldConversion<2> for Address {
    fn to_fields(&self) -> [Field; 2] {
        let pubkey = CompressedPubKey::from_address(std::str::from_utf8(&self.0).unwrap()).unwrap();

        let [field_0] = pubkey.x.to_fields();
        let [field_1] = pubkey.is_odd.to_fields();

        [field_0, field_1]
    }
}

impl ByteConversion<55> for Address {
    fn to_bytes(&self) -> [u8; 55] {
        self.0
    }

    fn from_bytes(bytes: &[u8; 55]) -> Self {
        Address(bytes.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_if_address_is_valid() {
        // /^B62q[1-9A-HJ-NP-Za-km-z]{51}$/

        let addr = "B62qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), true);

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq27vhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), false);

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2ivhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), false);

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2fjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), false);

        let addr = "B63qoTFrus93Ryi1VzbFakzErBBmcikHEq2ivhMkU4FfvhMkU4FfjGfCovv41fb";
        assert_eq!(Address::is_valid(addr), false);
    }
}
