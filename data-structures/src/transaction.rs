use crate::{Address, FromBytes, Signature, ToBytes, U256};

/// The enum that represents transaction types.
#[derive(Clone, Debug)]
pub enum Transaction {
    BurnToken {
        address: Address,
        signature: Signature,
        token_id: U256,
        token_amount: u64,
    },
    CreatePool {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount: u64,
    },
    ProvideLiquidity {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount_limit: u64,
    },
    RemoveLiquidity {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount_limit: u64,
        quote_token_amount_limit: u64,
        liquidity_point_amount: U256,
    },
    BuyTokens {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount_limit: u64,
    },
    SellTokens {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount_limit: u64,
        quote_token_amount: u64,
    },
}

impl From<[u8; 264]> for Transaction {
    fn from(value: [u8; 264]) -> Self {
        match value[0] {
            0 => Self::BurnToken {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                token_amount: u64::from_le_bytes(value[184..192].try_into().unwrap()),
            },
            1 => Self::CreatePool {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(value[184..216].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(value[216..224].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(value[224..232].try_into().unwrap()),
            },
            2 => Self::ProvideLiquidity {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(value[184..216].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(value[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(value[224..232].try_into().unwrap()),
            },
            3 => Self::RemoveLiquidity {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(value[184..216].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(value[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(value[224..232].try_into().unwrap()),
                liquidity_point_amount: U256::from_bytes(value[232..264].try_into().unwrap()),
            },
            4 => Self::BuyTokens {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(value[184..216].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(value[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(value[224..232].try_into().unwrap()),
            },
            _ => Self::SellTokens {
                address: Address::from_bytes(value[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(value[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(value[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(value[184..216].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(value[216..224].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(value[224..232].try_into().unwrap()),
            },
        }
    }
}

impl From<&Transaction> for [u8; 264] {
    fn from(value: &Transaction) -> Self {
        let mut buf = [0u8; 264];

        match value {
            Transaction::BurnToken {
                address,
                signature,
                token_id,
                token_amount,
            } => {
                buf[0] = 0;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&token_id.to_bytes());
                buf[184..192].copy_from_slice(&token_amount.to_bytes());
            }
            Transaction::CreatePool {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount,
            } => {
                buf[0] = 1;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&base_token_id.to_bytes());
                buf[184..216].copy_from_slice(&quote_token_id.to_bytes());
                buf[216..224].copy_from_slice(&base_token_amount.to_bytes());
                buf[224..232].copy_from_slice(&quote_token_amount.to_bytes());
            }
            Transaction::ProvideLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => {
                buf[0] = 2;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&base_token_id.to_bytes());
                buf[184..216].copy_from_slice(&quote_token_id.to_bytes());
                buf[216..224].copy_from_slice(&base_token_amount.to_bytes());
                buf[224..232].copy_from_slice(&quote_token_amount_limit.to_bytes());
            }
            Transaction::RemoveLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount_limit,
                liquidity_point_amount,
            } => {
                buf[0] = 3;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&base_token_id.to_bytes());
                buf[184..216].copy_from_slice(&quote_token_id.to_bytes());
                buf[216..224].copy_from_slice(&base_token_amount_limit.to_bytes());
                buf[224..232].copy_from_slice(&quote_token_amount_limit.to_bytes());
                buf[232..264].copy_from_slice(&liquidity_point_amount.to_bytes());
            }
            Transaction::BuyTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            } => {
                buf[0] = 4;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&base_token_id.to_bytes());
                buf[184..216].copy_from_slice(&quote_token_id.to_bytes());
                buf[216..224].copy_from_slice(&base_token_amount.to_bytes());
                buf[224..232].copy_from_slice(&quote_token_amount_limit.to_bytes());
            }
            Transaction::SellTokens {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount,
            } => {
                buf[0] = 5;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..152].copy_from_slice(&signature.to_bytes());
                buf[152..184].copy_from_slice(&base_token_id.to_bytes());
                buf[184..216].copy_from_slice(&quote_token_id.to_bytes());
                buf[216..224].copy_from_slice(&base_token_amount_limit.to_bytes());
                buf[224..232].copy_from_slice(&quote_token_amount.to_bytes());
            }
        }

        buf
    }
}
