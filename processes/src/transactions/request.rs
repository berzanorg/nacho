use nacho_data_structures::TxStatus;
use tokio::sync::oneshot;

pub enum Request {
    GetTxCount {
        sender: oneshot::Sender<Option<u64>>,
    },
    GetTxStatus {
        sender: oneshot::Sender<Option<TxStatus>>,
        tx_id: u64,
    },
    RejectTx {
        sender: oneshot::Sender<Option<()>>,
        tx_id: u64,
    },
    SetExecutedUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    SetProvedUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    SetSettledUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
}
