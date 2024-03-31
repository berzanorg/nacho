use primitive_types::U256 as PU256;
use std::ops::{Add, Div, Mul, Sub};

use ark_ff::{BigInteger256, PrimeField};

use crate::{Field, FromBytes, ToBytes};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct U256(pub [u8; 32]);

impl FromBytes for U256 {
    type Bytes = [u8; 32];

    fn from_bytes(bytes: Self::Bytes) -> Self {
        U256(bytes)
    }
}

impl ToBytes for U256 {
    type Bytes = [u8; 32];

    fn to_bytes(&self) -> Self::Bytes {
        self.0
    }
}

impl From<&U256> for Field {
    fn from(value: &U256) -> Self {
        let u64s: [u64; 4] = [
            u64::from_le_bytes(value.0[0..8].try_into().unwrap()),
            u64::from_le_bytes(value.0[8..16].try_into().unwrap()),
            u64::from_le_bytes(value.0[16..24].try_into().unwrap()),
            u64::from_le_bytes(value.0[24..32].try_into().unwrap()),
        ];

        let bigint = BigInteger256(u64s);

        let field: Field = bigint.into();

        field
    }
}

impl From<Field> for U256 {
    fn from(value: Field) -> Self {
        let mut buf = [0u8; 32];

        let bigint = value.into_repr();

        let u64s = bigint.0;

        buf[0..8].copy_from_slice(&u64s[0].to_le_bytes());
        buf[8..16].copy_from_slice(&u64s[1].to_le_bytes());
        buf[16..24].copy_from_slice(&u64s[2].to_le_bytes());
        buf[24..32].copy_from_slice(&u64s[3].to_le_bytes());

        let u256 = U256(buf);

        u256
    }
}

impl From<u64> for U256 {
    fn from(value: u64) -> Self {
        let mut buf = [0u8; 32];

        buf[0..8].copy_from_slice(&value.to_le_bytes());

        U256(buf)
    }
}

impl TryFrom<&U256> for u64 {
    type Error = ();

    fn try_from(value: &U256) -> Result<Self, Self::Error> {
        let buf = &value.0;

        for i in 8..32 {
            if buf[i] != 0 {
                return Err(());
            }
        }

        Ok(u64::from_le_bytes(buf[0..8].try_into().unwrap()))
    }
}

impl From<u128> for U256 {
    fn from(value: u128) -> Self {
        let mut buf = [0u8; 32];

        buf[0..16].copy_from_slice(&value.to_le_bytes());

        U256(buf)
    }
}

impl TryFrom<&U256> for u128 {
    type Error = ();

    fn try_from(value: &U256) -> Result<Self, Self::Error> {
        let buf = &value.0;

        for i in 16..32 {
            if buf[i] != 0 {
                return Err(());
            }
        }

        Ok(u128::from_le_bytes(buf[0..16].try_into().unwrap()))
    }
}

impl From<&PU256> for U256 {
    fn from(value: &PU256) -> Self {
        let mut buf = [0; 32];

        let u64s = value.0;

        buf[0..8].copy_from_slice(&u64s[0].to_le_bytes());
        buf[8..16].copy_from_slice(&u64s[1].to_le_bytes());
        buf[16..24].copy_from_slice(&u64s[2].to_le_bytes());
        buf[24..32].copy_from_slice(&u64s[3].to_le_bytes());

        U256(buf)
    }
}

impl From<&U256> for PU256 {
    fn from(value: &U256) -> Self {
        let mut buf = [0; 32];

        let u64s: [u64; 4] = [
            u64::from_le_bytes(value.0[0..8].try_into().unwrap()),
            u64::from_le_bytes(value.0[8..16].try_into().unwrap()),
            u64::from_le_bytes(value.0[16..24].try_into().unwrap()),
            u64::from_le_bytes(value.0[24..32].try_into().unwrap()),
        ];

        PU256(u64s)
    }
}

impl Add for U256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: PU256 = (&self).into();
        let y: PU256 = (&rhs).into();

        (&(x + y)).into()
    }
}

impl Sub for U256 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let x: PU256 = (&self).into();
        let y: PU256 = (&rhs).into();

        (&(x - y)).into()
    }
}

impl Mul for U256 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let x: PU256 = (&self).into();
        let y: PU256 = (&rhs).into();

        (&(x * y)).into()
    }
}

impl Div for U256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let x: PU256 = (&self).into();
        let y: PU256 = (&rhs).into();

        (&(x / y)).into()
    }
}
