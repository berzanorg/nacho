use nacho_data_structures::StatefulTransaction;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn push(&self, stateful_tx: StatefulTransaction) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::Push {
                sender: oneshot_sender,
                stateful_tx,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn pop(&self) -> Option<StatefulTransaction> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::Pop {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let maybe_stateful_tx = oneshot_receiver.await.ok()?;

        maybe_stateful_tx
    }
}
