use crate::{FromBytes, ToBytes};

/// Byte representation of base 58 encoded signatures.
pub struct Signature([u8; 96]);

impl Signature {
    // Example: 7mXM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92RfTecqcnSuPCspXehtawpCJjrBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW
    // Regexp: /^7mX[1-9A-HJ-NP-Za-km-z]{93}$/
    pub fn is_valid(sign: &str) -> Result<(), ()> {
        if sign.len() != 96 {
            return Err(());
        }

        if sign[0..3].as_bytes() != [55, 109, 88] {
            return Err(());
        }

        for c in sign.bytes() {
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

impl FromBytes for Signature {
    type Bytes = [u8; 96];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        Signature(bytes)
    }
}

impl ToBytes for Signature {
    type Bytes = [u8; 96];

    fn to_bytes(&self) -> Self::Bytes {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checks_if_signature_is_valid() {
        // /^7mX[1-9A-HJ-NP-Za-km-z]{93}$/

        let sign = "7mXM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92RfTecqcnSuPCspXehtawpCJjrBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW";
        assert_eq!(Signature::is_valid(sign), Ok(()));

        let sign = "7mcM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92RfTecqcnSuPCspXehtawpCJjrBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW";
        assert_eq!(Signature::is_valid(sign), Err(()));

        let sign = "7mXM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92IfTecqciSuPCspXihtawpCJjrBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW";
        assert_eq!(Signature::is_valid(sign), Err(()));

        let sign = "7mXM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92IfTepCJjrBJMnRW2jxBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW";
        assert_eq!(Signature::is_valid(sign), Err(()));

        let sign = "7mXM6pRXQCpjaqFuJ2omcZgvHwc6LybAqQwV92IfTecqciSuPCspXihtawpCJjrBJMnRW2jxBJMnRW2jxLd7zzqqckTUp9vzjrvCH2ghW";
        assert_eq!(Signature::is_valid(sign), Err(()));
    }
}
