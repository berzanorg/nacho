use nacho_data_structures::Transaction;
use tokio::sync::oneshot;

pub enum Request {
    Push {
        sender: oneshot::Sender<Option<()>>,
        transaction: Transaction,
    },
    Pop {
        sender: oneshot::Sender<Option<Transaction>>,
    },
}
