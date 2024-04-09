use crate::{ByteConversion, U256};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRoots {
    balances: U256,
    liquidities: U256,
    pools: U256,
    burns: U256,
}

impl ByteConversion<128> for StateRoots {
    fn to_bytes(&self) -> [u8; 128] {
        let mut buf = [0u8; 128];

        buf[0..32].copy_from_slice(&self.balances.to_bytes());
        buf[32..64].copy_from_slice(&self.liquidities.to_bytes());
        buf[64..96].copy_from_slice(&self.pools.to_bytes());
        buf[96..128].copy_from_slice(&self.burns.to_bytes());

        buf
    }

    fn from_bytes(bytes: &[u8; 128]) -> Self {
        Self {
            balances: U256::from_bytes(bytes[0..32].try_into().unwrap()),
            liquidities: U256::from_bytes(bytes[32..64].try_into().unwrap()),
            pools: U256::from_bytes(bytes[64..96].try_into().unwrap()),
            burns: U256::from_bytes(bytes[96..128].try_into().unwrap()),
        }
    }
}
