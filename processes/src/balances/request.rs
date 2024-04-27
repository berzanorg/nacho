use nacho_balances_db::{DoubleBalanceWitness, SingleBalanceWitness};
use nacho_data_structures::{Address, Balance, U256};
use tokio::sync::oneshot;

pub enum Request {
    GetBalance {
        sender: oneshot::Sender<Option<Balance>>,
        owner: Address,
        token_id: U256,
    },
    GetBalances {
        sender: oneshot::Sender<Option<Vec<Balance>>>,
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
        balance: Balance,
    },
    UpdateBalance {
        sender: oneshot::Sender<Option<()>>,
        balance: Balance,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        balance: Balance,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        balance: Balance,
    },
    GetRoot {
        sender: oneshot::Sender<Option<U256>>,
    },
}
