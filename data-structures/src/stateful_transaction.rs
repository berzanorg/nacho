use crate::{
    transaction::{
        BurnTokensTransaction, BuyTokensTransaction, CreatePoolTransaction,
        DepositTokensTransaction, ProvideLiquidityTransaction, RemoveLiquidityTransaction,
        SellTokensTransaction,
    },
    U256,
};

/// The enum that represents stateful transaction types.
///
/// Stateful transactions are used to simulate the same conditions the transactions were executed in by storing the related state alongside the transaction.
///
#[derive(Clone, Debug)]
pub enum StatefulTransaction {
    CreateGenesis {
        state: (),
        transaction: (),
    },
    DepositTokens {
        state: DepositTokensTransactionState,
        transaction: DepositTokensTransaction,
    },
    BurnTokens {
        state: BurnTokensTransactionState,
        transaction: BurnTokensTransaction,
    },
    CreatePool {
        state: CreatePoolTransactionState,
        transaction: CreatePoolTransaction,
    },
    ProvideLiquidity {
        state: ProvideLiquidityTransactionState,
        transaction: ProvideLiquidityTransaction,
    },
    RemoveLiquidity {
        state: RemoveLiquidityTransactionState,
        transaction: RemoveLiquidityTransaction,
    },
    BuyTokens {
        state: BuyTokensTransactionState,
        transaction: BuyTokensTransaction,
    },
    SellTokens {
        state: SellTokensTransactionState,
        transaction: SellTokensTransaction,
    },
}

#[derive(Clone, Debug)]
pub struct DepositTokensTransactionState {
    pub user_token_balance: u64,
    pub is_first_deposit_of_token: bool,
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
    pub is_first_providing: bool,
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
