use nacho_data_structures::U256;
use nacho_withdrawals_db::SingleWithdrawalWitness;
use tokio::sync::oneshot;

pub enum Request {
    GetWitness {
        sender: oneshot::Sender<Option<SingleWithdrawalWitness>>,
        burn_id: u64,
    },
    SetLeaf {
        sender: oneshot::Sender<Option<()>>,
        burn_id: u64,
        value: U256,
    },
}
