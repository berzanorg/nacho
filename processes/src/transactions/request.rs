use nacho_data_structures::TxStatus;
use tokio::sync::oneshot;

pub enum Request {
    GetTotalTxCount {
        sender: oneshot::Sender<Option<u64>>,
    },
    GetTxStatus {
        sender: oneshot::Sender<Option<TxStatus>>,
        tx_id: u64,
    },
    AddNewTx {
        sender: oneshot::Sender<Option<u64>>,
    },
    SetRejected {
        sender: oneshot::Sender<Option<()>>,
        tx_id: u64,
    },
    SetExecutedUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    GetExecutedUntil {
        sender: oneshot::Sender<Option<u64>>,
    },
    SetProvedUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    GetProvedUntil {
        sender: oneshot::Sender<Option<u64>>,
    },
    SetSettledUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    GetSettledUntil {
        sender: oneshot::Sender<Option<u64>>,
    },
    SetMergedUntil {
        sender: oneshot::Sender<Option<()>>,
        until_tx_id: u64,
    },
    GetMergedUntil {
        sender: oneshot::Sender<Option<u64>>,
    },
}
