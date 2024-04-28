use nacho_data_structures::{Withdrawal, U256};
use nacho_withdrawals_db::SingleWithdrawalWitness;
use tokio::sync::oneshot;

pub enum Request {
    Set {
        sender: oneshot::Sender<Option<()>>,
        index: u64,
        withdrawal: Withdrawal,
    },
    Get {
        sender: oneshot::Sender<Option<Withdrawal>>,
        index: u64,
    },
    GetRoot {
        sender: oneshot::Sender<Option<U256>>,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SingleWithdrawalWitness>>,
        index: u64,
    },
}
