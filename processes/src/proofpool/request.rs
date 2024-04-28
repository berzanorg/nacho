use nacho_data_structures::StatefulTransaction;
use tokio::sync::oneshot;

pub enum Request {
    Push {
        sender: oneshot::Sender<Option<()>>,
        stateful_tx: StatefulTransaction,
    },
    Pop {
        sender: oneshot::Sender<Option<StatefulTransaction>>,
    },
}
