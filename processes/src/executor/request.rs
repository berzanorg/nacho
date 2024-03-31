use nacho_data_structures::Transaction;
use tokio::sync::oneshot;

pub enum Request {
    ExecuteNext { sender: oneshot::Sender<Option<()>> },
}
