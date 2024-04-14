use nacho_data_structures::Transaction;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn check_signature(&self, tx: Transaction) -> Option<bool> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::CheckSignature {
                sender: oneshot_sender,
                tx,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
