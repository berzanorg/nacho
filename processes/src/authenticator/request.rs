use nacho_data_structures::Transaction;
use tokio::sync::oneshot;

pub enum Request {
    CheckSignature {
        sender: oneshot::Sender<Option<bool>>,
        tx: Transaction,
    },
}
