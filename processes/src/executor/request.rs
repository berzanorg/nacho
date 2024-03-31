use tokio::sync::oneshot;

pub enum Request {
    ExecuteNext { sender: oneshot::Sender<Option<()>> },
}
