use nacho_balances_db::{DoubleBalanceWitness, SingleBalanceWitness};
use nacho_data_structures::{Address, U256};
use tokio::sync::oneshot;

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;

pub enum Request {
    GetTokenAmount {
        sender: oneshot::Sender<Option<TokenAmount>>,
        owner: Address,
        token_id: U256,
    },
    GetBalances {
        sender: oneshot::Sender<Option<Vec<(TokenId, TokenAmount)>>>,
        owner: Address,
    },
    GetSingleWitness {
        sender: oneshot::Sender<Option<SingleBalanceWitness>>,
        owner: Address,
        token_id: U256,
    },
    GetDoubleWitness {
        sender: oneshot::Sender<Option<DoubleBalanceWitness>>,
        owner: Address,
        base_token_id: U256,
        quote_token_id: U256,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SingleBalanceWitness>>,
    },
    PushBalance {
        sender: oneshot::Sender<Option<()>>,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    },
    UpdateBalance {
        sender: oneshot::Sender<Option<()>>,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        owner: Address,
        token_id: U256,
        token_amount: u64,
    },
}
