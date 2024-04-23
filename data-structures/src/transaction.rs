use crate::{Address, ByteConversion, Signature, U256};

/// The enum that represents transaction types.
///
/// Transactions are used to describe what to execute and how to modify the rollup state by storing the required information.
#[derive(Clone, Debug)]
pub enum Transaction {
    CreateGenesis(),
    DepositTokens(DepositTokensTransaction),
    BurnTokens(BurnTokensTransaction),
    CreatePool(CreatePoolTransaction),
    ProvideLiquidity(ProvideLiquidityTransaction),
    RemoveLiquidity(RemoveLiquidityTransaction),
    BuyTokens(BuyTokensTransaction),
    SellTokens(SellTokensTransaction),
}

#[derive(Clone, Debug)]
pub struct DepositTokensTransaction {
    pub user_address: Address,
    pub token_id: U256,
    pub token_amount: u64,
}

#[derive(Clone, Debug)]
pub struct BurnTokensTransaction {
    pub address: Address,
    pub signature: Signature,
    pub token_id: U256,
    pub token_amount: u64,
}

#[derive(Clone, Debug)]
pub struct CreatePoolTransaction {
    pub address: Address,
    pub signature: Signature,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount: u64,
    pub quote_token_amount: u64,
}

#[derive(Clone, Debug)]
pub struct ProvideLiquidityTransaction {
    pub address: Address,
    pub signature: Signature,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount: u64,
    pub quote_token_amount_limit: u64,
}

#[derive(Clone, Debug)]
pub struct RemoveLiquidityTransaction {
    pub address: Address,
    pub signature: Signature,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount_limit: u64,
    pub quote_token_amount_limit: u64,
    pub points: U256,
}

#[derive(Clone, Debug)]
pub struct BuyTokensTransaction {
    pub address: Address,
    pub signature: Signature,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount: u64,
    pub quote_token_amount_limit: u64,
}

#[derive(Clone, Debug)]
pub struct SellTokensTransaction {
    pub address: Address,
    pub signature: Signature,
    pub base_token_id: U256,
    pub quote_token_id: U256,
    pub base_token_amount_limit: u64,
    pub quote_token_amount: u64,
}

impl ByteConversion<232> for Transaction {
    fn to_bytes(&self) -> [u8; 232] {
        let mut buf = [0u8; 232];

        match self {
            Transaction::CreateGenesis {} => {
                buf[0] = 0;
            }
            Transaction::DepositTokens(DepositTokensTransaction {
                user_address,
                token_id,
                token_amount,
            }) => {
                buf[0] = 1;
                buf[1..56].copy_from_slice(&user_address.to_bytes());
                buf[56..88].copy_from_slice(&token_id.to_bytes());
                buf[88..96].copy_from_slice(&token_amount.to_bytes());
            }
            Transaction::BurnTokens(BurnTokensTransaction {
                address,
                signature,
                token_id,
                token_amount,
            }) => {
                buf[0] = 2;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&token_id.to_bytes());
                buf[152..160].copy_from_slice(&token_amount.to_bytes());
            }
            Transaction::CreatePool(CreatePoolTransaction {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount,
            }) => {
                buf[0] = 3;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount.to_bytes());
            }
            Transaction::ProvideLiquidity(ProvideLiquidityTransaction {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            }) => {
                buf[0] = 4;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
            }
            Transaction::RemoveLiquidity(RemoveLiquidityTransaction {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount_limit,
                points: liquidity_point_amount,
            }) => {
                buf[0] = 5;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount_limit.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
                buf[200..232].copy_from_slice(&liquidity_point_amount.to_bytes());
            }
            Transaction::BuyTokens(BuyTokensTransaction {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount_limit,
            }) => {
                buf[0] = 6;
                buf[1..56].copy_from_slice(&address.to_bytes());
                buf[56..120].copy_from_slice(&signature.to_bytes());
                buf[120..152].copy_from_slice(&base_token_id.to_bytes());
                buf[152..184].copy_from_slice(&quote_token_id.to_bytes());
                buf[184..192].copy_from_slice(&base_token_amount.to_bytes());
                buf[192..200].copy_from_slice(&quote_token_amount_limit.to_bytes());
            }
            Transaction::SellTokens(SellTokensTransaction {
                address,
                signature,
                base_token_id,
                quote_token_id,
                base_token_amount_limit,
                quote_token_amount,
            }) => {
                buf[0] = 7;
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
            0 => Self::CreateGenesis(),
            1 => Self::DepositTokens(DepositTokensTransaction {
                user_address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                token_id: U256::from_bytes(bytes[56..88].try_into().unwrap()),
                token_amount: u64::from_le_bytes(bytes[88..96].try_into().unwrap()),
            }),
            2 => Self::BurnTokens(BurnTokensTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                token_amount: u64::from_le_bytes(bytes[152..160].try_into().unwrap()),
            }),
            3 => Self::CreatePool(CreatePoolTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            }),
            4 => Self::ProvideLiquidity(ProvideLiquidityTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            }),
            5 => Self::RemoveLiquidity(RemoveLiquidityTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
                points: U256::from_bytes(bytes[200..232].try_into().unwrap()),
            }),
            6 => Self::BuyTokens(BuyTokensTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount_limit: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            }),
            _ => Self::SellTokens(SellTokensTransaction {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..120].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[120..152].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                base_token_amount_limit: u64::from_le_bytes(bytes[184..192].try_into().unwrap()),
                quote_token_amount: u64::from_le_bytes(bytes[192..200].try_into().unwrap()),
            }),
        }
    }
}
