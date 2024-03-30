use nacho_data_structures::Transaction;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn push(&self, transaction: Transaction) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::Push {
                sender: oneshot_sender,
                transaction,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn pop(&self) -> Option<Transaction> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::Pop {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let maybe_transaction = oneshor_receiver.await.ok()?;

        maybe_transaction
    }
}
