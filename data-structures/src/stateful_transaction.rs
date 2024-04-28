use crate::{
    transaction::{
        BurnTokensTransaction, BuyTokensTransaction, CreatePoolTransaction,
        DepositTokensTransaction, ProvideLiquidityTransaction, RemoveLiquidityTransaction,
        SellTokensTransaction,
    },
    ByteConversion, U256,
};

/// The enum that represents stateful transaction types.
///
/// Stateful transactions are used to simulate the same conditions the transactions were executed in by storing the related state alongside the transaction.
///
#[derive(Clone, Debug)]
pub enum StatefulTransaction {
    CreateGenesis {
        transaction: (),
        state: (),
    },
    DepositTokens {
        transaction: DepositTokensTransaction,
        state: DepositTokensTransactionState,
    },
    BurnTokens {
        transaction: BurnTokensTransaction,
        state: BurnTokensTransactionState,
    },
    CreatePool {
        transaction: CreatePoolTransaction,
        state: CreatePoolTransactionState,
    },
    ProvideLiquidity {
        transaction: ProvideLiquidityTransaction,
        state: ProvideLiquidityTransactionState,
    },
    RemoveLiquidity {
        transaction: RemoveLiquidityTransaction,
        state: RemoveLiquidityTransactionState,
    },
    BuyTokens {
        transaction: BuyTokensTransaction,
        state: BuyTokensTransactionState,
    },
    SellTokens {
        transaction: SellTokensTransaction,
        state: SellTokensTransactionState,
    },
}

#[derive(Clone, Debug)]
pub struct DepositTokensTransactionState {
    pub user_token_balance: u64,
}

#[derive(Clone, Debug)]
pub struct BurnTokensTransactionState {
    pub user_burn_token_amount: u64,
    pub user_balance_token_amount: u64,
}

#[derive(Clone, Debug)]
pub struct CreatePoolTransactionState {
    pub user_balance_base_token_amount: u64,
    pub user_balance_quote_token_amount: u64,
}

#[derive(Clone, Debug)]
pub struct ProvideLiquidityTransactionState {
    pub user_liquidity_points: U256,
    pub user_balance_base_token_amount: u64,
    pub user_balance_quote_token_amount: u64,
    pub pool_base_token_amount: u64,
    pub pool_quote_token_amount: u64,
    pub pool_total_liquidity_points: U256,
}

#[derive(Clone, Debug)]
pub struct RemoveLiquidityTransactionState {
    pub user_liquidity_points: U256,
    pub user_balance_base_token_amount: u64,
    pub user_balance_quote_token_amount: u64,
    pub pool_base_token_amount: u64,
    pub pool_quote_token_amount: u64,
    pub pool_total_liquidity_points: U256,
}

#[derive(Clone, Debug)]
pub struct BuyTokensTransactionState {
    pub user_balance_base_token_amount: u64,
    pub user_balance_quote_token_amount: u64,
    pub pool_base_token_amount: u64,
    pub pool_quote_token_amount: u64,
    pub pool_total_liquidity_points: U256,
}

#[derive(Clone, Debug)]
pub struct SellTokensTransactionState {
    pub user_balance_base_token_amount: u64,
    pub user_balance_quote_token_amount: u64,
    pub pool_base_token_amount: u64,
    pub pool_quote_token_amount: u64,
    pub pool_total_liquidity_points: U256,
}

impl ByteConversion<328> for StatefulTransaction {
    fn to_bytes(&self) -> [u8; 328] {
        let mut buf = [0u8; 328];
        match self {
            StatefulTransaction::CreateGenesis {
                transaction: _,
                state: _,
            } => {
                buf[0] = 0;
            }
            StatefulTransaction::DepositTokens { transaction, state } => {
                buf[0] = 1;
                buf[1..96].copy_from_slice(&transaction.to_bytes());
                buf[96..104].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::BurnTokens { transaction, state } => {
                buf[0] = 2;
                buf[1..160].copy_from_slice(&transaction.to_bytes());
                buf[160..176].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::CreatePool { transaction, state } => {
                buf[0] = 3;
                buf[1..200].copy_from_slice(&transaction.to_bytes());
                buf[200..216].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::ProvideLiquidity { transaction, state } => {
                buf[0] = 4;
                buf[1..200].copy_from_slice(&transaction.to_bytes());
                buf[200..296].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::RemoveLiquidity { transaction, state } => {
                buf[0] = 5;
                buf[1..232].copy_from_slice(&transaction.to_bytes());
                buf[232..328].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::BuyTokens { transaction, state } => {
                buf[0] = 6;
                buf[1..200].copy_from_slice(&transaction.to_bytes());
                buf[200..264].copy_from_slice(&state.to_bytes());
            }
            StatefulTransaction::SellTokens { transaction, state } => {
                buf[0] = 7;
                buf[1..200].copy_from_slice(&transaction.to_bytes());
                buf[200..264].copy_from_slice(&state.to_bytes());
            }
        }
        buf
    }

    fn from_bytes(bytes: &[u8; 328]) -> Self {
        match bytes[0] {
            0 => StatefulTransaction::CreateGenesis {
                transaction: (),
                state: (),
            },
            1 => StatefulTransaction::DepositTokens {
                transaction: DepositTokensTransaction::from_bytes(bytes[1..96].try_into().unwrap()),
                state: DepositTokensTransactionState::from_bytes(
                    bytes[96..104].try_into().unwrap(),
                ),
            },
            2 => StatefulTransaction::BurnTokens {
                transaction: BurnTokensTransaction::from_bytes(bytes[1..160].try_into().unwrap()),
                state: BurnTokensTransactionState::from_bytes(bytes[160..176].try_into().unwrap()),
            },
            3 => StatefulTransaction::CreatePool {
                transaction: CreatePoolTransaction::from_bytes(bytes[1..200].try_into().unwrap()),
                state: CreatePoolTransactionState::from_bytes(bytes[200..216].try_into().unwrap()),
            },
            4 => StatefulTransaction::ProvideLiquidity {
                transaction: ProvideLiquidityTransaction::from_bytes(
                    bytes[1..200].try_into().unwrap(),
                ),
                state: ProvideLiquidityTransactionState::from_bytes(
                    bytes[200..296].try_into().unwrap(),
                ),
            },
            5 => StatefulTransaction::RemoveLiquidity {
                transaction: RemoveLiquidityTransaction::from_bytes(
                    bytes[1..232].try_into().unwrap(),
                ),
                state: RemoveLiquidityTransactionState::from_bytes(
                    bytes[232..328].try_into().unwrap(),
                ),
            },
            6 => StatefulTransaction::BuyTokens {
                transaction: BuyTokensTransaction::from_bytes(bytes[1..200].try_into().unwrap()),
                state: BuyTokensTransactionState::from_bytes(bytes[200..264].try_into().unwrap()),
            },
            _ => StatefulTransaction::SellTokens {
                transaction: SellTokensTransaction::from_bytes(bytes[1..200].try_into().unwrap()),
                state: SellTokensTransactionState::from_bytes(bytes[200..264].try_into().unwrap()),
            },
        }
    }
}

impl ByteConversion<8> for DepositTokensTransactionState {
    fn to_bytes(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        buf[0..8].copy_from_slice(&self.user_token_balance.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 8]) -> Self {
        DepositTokensTransactionState {
            user_token_balance: u64::from_bytes(bytes[0..8].try_into().unwrap()),
        }
    }
}

impl ByteConversion<16> for BurnTokensTransactionState {
    fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&self.user_burn_token_amount.to_bytes());
        buf[8..16].copy_from_slice(&self.user_balance_token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 16]) -> Self {
        BurnTokensTransactionState {
            user_burn_token_amount: u64::from_bytes(bytes[0..8].try_into().unwrap()),
            user_balance_token_amount: u64::from_bytes(bytes[8..16].try_into().unwrap()),
        }
    }
}

impl ByteConversion<16> for CreatePoolTransactionState {
    fn to_bytes(&self) -> [u8; 16] {
        let mut buf = [0u8; 16];
        buf[0..8].copy_from_slice(&self.user_balance_base_token_amount.to_bytes());
        buf[8..16].copy_from_slice(&self.user_balance_quote_token_amount.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 16]) -> Self {
        CreatePoolTransactionState {
            user_balance_base_token_amount: u64::from_bytes(bytes[0..8].try_into().unwrap()),
            user_balance_quote_token_amount: u64::from_bytes(bytes[8..16].try_into().unwrap()),
        }
    }
}

impl ByteConversion<96> for ProvideLiquidityTransactionState {
    fn to_bytes(&self) -> [u8; 96] {
        let mut buf = [0u8; 96];
        buf[0..32].copy_from_slice(&self.user_liquidity_points.to_bytes());
        buf[32..40].copy_from_slice(&self.user_balance_base_token_amount.to_bytes());
        buf[40..48].copy_from_slice(&self.user_balance_quote_token_amount.to_bytes());
        buf[48..56].copy_from_slice(&self.pool_base_token_amount.to_bytes());
        buf[56..64].copy_from_slice(&self.pool_quote_token_amount.to_bytes());
        buf[64..96].copy_from_slice(&self.pool_total_liquidity_points.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 96]) -> Self {
        ProvideLiquidityTransactionState {
            user_liquidity_points: U256::from_bytes(bytes[0..32].try_into().unwrap()),
            user_balance_base_token_amount: u64::from_bytes(bytes[32..40].try_into().unwrap()),
            user_balance_quote_token_amount: u64::from_bytes(bytes[40..48].try_into().unwrap()),
            pool_base_token_amount: u64::from_bytes(bytes[48..56].try_into().unwrap()),
            pool_quote_token_amount: u64::from_bytes(bytes[56..64].try_into().unwrap()),
            pool_total_liquidity_points: U256::from_bytes(bytes[64..96].try_into().unwrap()),
        }
    }
}

impl ByteConversion<96> for RemoveLiquidityTransactionState {
    fn to_bytes(&self) -> [u8; 96] {
        let mut buf = [0u8; 96];
        buf[0..32].copy_from_slice(&self.user_liquidity_points.to_bytes());
        buf[32..40].copy_from_slice(&self.user_balance_base_token_amount.to_bytes());
        buf[40..48].copy_from_slice(&self.user_balance_quote_token_amount.to_bytes());
        buf[48..56].copy_from_slice(&self.pool_base_token_amount.to_bytes());
        buf[56..64].copy_from_slice(&self.pool_quote_token_amount.to_bytes());
        buf[64..96].copy_from_slice(&self.pool_total_liquidity_points.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 96]) -> Self {
        RemoveLiquidityTransactionState {
            user_liquidity_points: U256::from_bytes(bytes[0..32].try_into().unwrap()),
            user_balance_base_token_amount: u64::from_bytes(bytes[32..40].try_into().unwrap()),
            user_balance_quote_token_amount: u64::from_bytes(bytes[40..48].try_into().unwrap()),
            pool_base_token_amount: u64::from_bytes(bytes[48..56].try_into().unwrap()),
            pool_quote_token_amount: u64::from_bytes(bytes[56..64].try_into().unwrap()),
            pool_total_liquidity_points: U256::from_bytes(bytes[64..96].try_into().unwrap()),
        }
    }
}

impl ByteConversion<64> for BuyTokensTransactionState {
    fn to_bytes(&self) -> [u8; 64] {
        let mut buf = [0u8; 64];
        buf[0..8].copy_from_slice(&self.user_balance_base_token_amount.to_bytes());
        buf[8..16].copy_from_slice(&self.user_balance_quote_token_amount.to_bytes());
        buf[16..24].copy_from_slice(&self.pool_base_token_amount.to_bytes());
        buf[24..32].copy_from_slice(&self.pool_quote_token_amount.to_bytes());
        buf[32..64].copy_from_slice(&self.pool_total_liquidity_points.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 64]) -> Self {
        BuyTokensTransactionState {
            user_balance_base_token_amount: u64::from_bytes(bytes[0..8].try_into().unwrap()),
            user_balance_quote_token_amount: u64::from_bytes(bytes[8..16].try_into().unwrap()),
            pool_base_token_amount: u64::from_bytes(bytes[16..24].try_into().unwrap()),
            pool_quote_token_amount: u64::from_bytes(bytes[24..32].try_into().unwrap()),
            pool_total_liquidity_points: U256::from_bytes(bytes[32..64].try_into().unwrap()),
        }
    }
}

impl ByteConversion<64> for SellTokensTransactionState {
    fn to_bytes(&self) -> [u8; 64] {
        let mut buf = [0u8; 64];
        buf[0..8].copy_from_slice(&self.user_balance_base_token_amount.to_bytes());
        buf[8..16].copy_from_slice(&self.user_balance_quote_token_amount.to_bytes());
        buf[16..24].copy_from_slice(&self.pool_base_token_amount.to_bytes());
        buf[24..32].copy_from_slice(&self.pool_quote_token_amount.to_bytes());
        buf[32..64].copy_from_slice(&self.pool_total_liquidity_points.to_bytes());
        buf
    }

    fn from_bytes(bytes: &[u8; 64]) -> Self {
        SellTokensTransactionState {
            user_balance_base_token_amount: u64::from_bytes(bytes[0..8].try_into().unwrap()),
            user_balance_quote_token_amount: u64::from_bytes(bytes[8..16].try_into().unwrap()),
            pool_base_token_amount: u64::from_bytes(bytes[16..24].try_into().unwrap()),
            pool_quote_token_amount: u64::from_bytes(bytes[24..32].try_into().unwrap()),
            pool_total_liquidity_points: U256::from_bytes(bytes[32..64].try_into().unwrap()),
        }
    }
}
