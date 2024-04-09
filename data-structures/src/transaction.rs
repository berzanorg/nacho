use crate::{Address, ByteConversion, Signature, U256};

/// The enum that represents transaction types.
#[derive(Clone, Debug)]
pub enum Transaction {
    BurnTokens {
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
        points: U256,
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

impl ByteConversion<232> for Transaction {
    fn to_bytes(&self) -> [u8; 232] {
        let mut buf = [0u8; 232];

        match self {
            Transaction::BurnTokens {
                address,
                signature,
                token_id,
                token_amount,
            } => {
                buf[0] = 0;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&token_id.to_bytes());
                buf[152..160].copy_from_slice(&token_amount.to_bytes());
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
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount.to_bytes());
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
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
            }
            Transaction::RemoveLiquidity {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount_limit,
                points: liquidity_point_amount,
            } => {
                buf[0] = 3;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount_limit.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
                buf[200..232].copy_from_slice(&liquidity_point_amount.to_bytes());
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
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
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
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount_limit.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount.to_bytes());
            }
        }

        buf
    }

    fn from_bytes(bytes: &[u8; 232]) -> Self {
        match bytes[0] {
            0 => Self::BurnTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                token_amount: u64::from_le_bytes(bytes[152..160].try_into().unwrap()),
            },
            1 => Self::CreatePool {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            },
            2 => Self::ProvideLiquidity {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            },
            3 => Self::RemoveLiquidity {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
                points: U256::from_bytes(bytes[200..232].try_into().unwrap()),
            },
            4 => Self::BuyTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            },
            _ => Self::SellTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            },
        }
    }
}
