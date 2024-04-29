use nacho_data_structures::TxStatus;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_total_tx_count(&self) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::GetTotalTxCount {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let tx_count = oneshot_receiver.await.ok()?;

        tx_count
    }

    pub async fn get_tx_status(&self, tx_id: u64) -> Option<TxStatus> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::GetTxStatus {
                sender: oneshot_sender,
                tx_id,
            })
            .await
            .ok()?;

        let tx_status = oneshot_receiver.await.ok()?;

        tx_status
    }

    pub async fn add_new_tx(&self) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::AddNewTx {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn set_rejected(&self, tx_id: u64) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::SetRejected {
                sender: oneshot_sender,
                tx_id,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn set_executed_until(&self, until_tx_id: u64) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::SetExecutedUntil {
                sender: oneshot_sender,
                until_tx_id,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get_executed_until(&self) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::GetExecutedUntil {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn set_proved_until(&self, until_tx_id: u64) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::SetProvedUntil {
                sender: oneshot_sender,
                until_tx_id,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get_proved_until(&self) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::GetProvedUntil {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn set_settled_until(&self, until_tx_id: u64) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::SetSettledUntil {
                sender: oneshot_sender,
                until_tx_id,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get_settled_until(&self) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();
        self.sender
            .send(Request::GetSettledUntil {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
