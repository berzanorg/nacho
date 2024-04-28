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
            Transaction::DepositTokens(deposit_tokens_transaction) => {
                buf[0] = 1;
                buf[1..96].copy_from_slice(&deposit_tokens_transaction.to_bytes());
            }
            Transaction::BurnTokens(burn_tokens_transaction) => {
                buf[0] = 2;
                buf[1..160].copy_from_slice(&burn_tokens_transaction.to_bytes());
            }
            Transaction::CreatePool(create_pool_transaction) => {
                buf[0] = 3;
                buf[1..200].copy_from_slice(&create_pool_transaction.to_bytes());
            }
            Transaction::ProvideLiquidity(provide_liquidity_transaction) => {
                buf[0] = 4;
                buf[1..200].copy_from_slice(&provide_liquidity_transaction.to_bytes());
            }
            Transaction::RemoveLiquidity(remove_liquidity_transaction) => {
                buf[0] = 5;
                buf[1..232].copy_from_slice(&remove_liquidity_transaction.to_bytes());
            }
            Transaction::BuyTokens(buy_tokens_transaction) => {
                buf[0] = 6;
                buf[1..200].copy_from_slice(&buy_tokens_transaction.to_bytes());
            }
            Transaction::SellTokens(sell_tokens_transaction) => {
                buf[0] = 7;
                buf[1..200].copy_from_slice(&sell_tokens_transaction.to_bytes());
            }
        }

        buf
    }

    fn from_bytes(bytes: &[u8; 232]) -> Self {
        match bytes[0] {
            0 => Self::CreateGenesis(),
            1 => Self::DepositTokens(DepositTokensTransaction::from_bytes(
                bytes[1..96].try_into().unwrap(),
            )),
            2 => Self::BurnTokens(BurnTokensTransaction::from_bytes(
                bytes[1..160].try_into().unwrap(),
            )),
            3 => Self::CreatePool(CreatePoolTransaction::from_bytes(
                bytes[1..200].try_into().unwrap(),
            )),
            4 => Self::ProvideLiquidity(ProvideLiquidityTransaction::from_bytes(
                bytes[1..200].try_into().unwrap(),
            )),
            5 => Self::RemoveLiquidity(RemoveLiquidityTransaction::from_bytes(
                bytes[1..232].try_into().unwrap(),
            )),
            6 => Self::BuyTokens(BuyTokensTransaction::from_bytes(
                bytes[1..200].try_into().unwrap(),
            )),
            _ => Self::SellTokens(SellTokensTransaction::from_bytes(
                bytes[1..200].try_into().unwrap(),
            )),
        }
    }
}

impl ByteConversion<95> for DepositTokensTransaction {
    fn to_bytes(&self) -> [u8; 95] {
        let mut buf = [0u8; 95];
        buf[0..55].copy_from_slice(&self.user_address.to_bytes());
        buf[55..87].copy_from_slice(&self.token_id.to_bytes());
        buf[87..95].copy_from_slice(&self.token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 95]) -> Self {
        DepositTokensTransaction {
            user_address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            token_id: U256::from_bytes(bytes[55..87].try_into().unwrap()),
            token_amount: u64::from_bytes(bytes[87..95].try_into().unwrap()),
        }
    }
}

impl ByteConversion<159> for BurnTokensTransaction {
    fn to_bytes(&self) -> [u8; 159] {
        let mut buf = [0u8; 159];
        buf[0..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.token_id.to_bytes());
        buf[151..159].copy_from_slice(&self.token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 159]) -> Self {
        BurnTokensTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..119].try_into().unwrap()),
            token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            token_amount: u64::from_bytes(bytes[151..159].try_into().unwrap()),
        }
    }
}

impl ByteConversion<199> for CreatePoolTransaction {
    fn to_bytes(&self) -> [u8; 199] {
        let mut buf = [0u8; 199];
        buf[0..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.base_token_id.to_bytes());
        buf[151..183].copy_from_slice(&self.quote_token_id.to_bytes());
        buf[183..191].copy_from_slice(&self.base_token_amount.to_bytes());
        buf[191..199].copy_from_slice(&self.quote_token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 199]) -> Self {
        CreatePoolTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..119].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[151..183].try_into().unwrap()),
            base_token_amount: u64::from_bytes(bytes[183..191].try_into().unwrap()),
            quote_token_amount: u64::from_bytes(bytes[191..199].try_into().unwrap()),
        }
    }
}

impl ByteConversion<199> for ProvideLiquidityTransaction {
    fn to_bytes(&self) -> [u8; 199] {
        let mut buf = [0u8; 199];
        buf[0..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.base_token_id.to_bytes());
        buf[151..183].copy_from_slice(&self.quote_token_id.to_bytes());
        buf[183..191].copy_from_slice(&self.base_token_amount.to_bytes());
        buf[191..199].copy_from_slice(&self.quote_token_amount_limit.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 199]) -> Self {
        ProvideLiquidityTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..119].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[151..183].try_into().unwrap()),
            base_token_amount: u64::from_bytes(bytes[183..191].try_into().unwrap()),
            quote_token_amount_limit: u64::from_bytes(bytes[191..199].try_into().unwrap()),
        }
    }
}

impl ByteConversion<231> for RemoveLiquidityTransaction {
    fn to_bytes(&self) -> [u8; 231] {
        let mut buf = [0u8; 231];
        buf[0..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.base_token_id.to_bytes());
        buf[151..183].copy_from_slice(&self.quote_token_id.to_bytes());
        buf[183..191].copy_from_slice(&self.base_token_amount_limit.to_bytes());
        buf[191..199].copy_from_slice(&self.quote_token_amount_limit.to_bytes());
        buf[199..231].copy_from_slice(&self.points.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 231]) -> Self {
        RemoveLiquidityTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..119].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[151..183].try_into().unwrap()),
            base_token_amount_limit: u64::from_bytes(bytes[183..191].try_into().unwrap()),
            quote_token_amount_limit: u64::from_bytes(bytes[191..199].try_into().unwrap()),
            points: U256::from_bytes(bytes[199..131].try_into().unwrap()),
        }
    }
}

impl ByteConversion<199> for BuyTokensTransaction {
    fn to_bytes(&self) -> [u8; 199] {
        let mut buf = [0u8; 199];
        buf[1..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.base_token_id.to_bytes());
        buf[151..183].copy_from_slice(&self.quote_token_id.to_bytes());
        buf[183..191].copy_from_slice(&self.base_token_amount.to_bytes());
        buf[191..199].copy_from_slice(&self.quote_token_amount_limit.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 199]) -> Self {
        BuyTokensTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..119].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[151..183].try_into().unwrap()),
            base_token_amount: u64::from_bytes(bytes[183..191].try_into().unwrap()),
            quote_token_amount_limit: u64::from_bytes(bytes[191..199].try_into().unwrap()),
        }
    }
}

impl ByteConversion<199> for SellTokensTransaction {
    fn to_bytes(&self) -> [u8; 199] {
        let mut buf = [0u8; 199];
        buf[0..55].copy_from_slice(&self.address.to_bytes());
        buf[55..119].copy_from_slice(&self.signature.to_bytes());
        buf[119..151].copy_from_slice(&self.base_token_id.to_bytes());
        buf[151..193].copy_from_slice(&self.quote_token_id.to_bytes());
        buf[183..191].copy_from_slice(&self.base_token_amount_limit.to_bytes());
        buf[191..199].copy_from_slice(&self.quote_token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 199]) -> Self {
        SellTokensTransaction {
            address: Address::from_bytes(bytes[0..55].try_into().unwrap()),
            signature: Signature::from_bytes(bytes[55..199].try_into().unwrap()),
            base_token_id: U256::from_bytes(bytes[119..151].try_into().unwrap()),
            quote_token_id: U256::from_bytes(bytes[151..183].try_into().unwrap()),
            base_token_amount_limit: u64::from_bytes(bytes[183..191].try_into().unwrap()),
            quote_token_amount: u64::from_bytes(bytes[191..199].try_into().unwrap()),
        }
    }
}
