use nacho_data_structures::{Withdrawal, U256};
use nacho_withdrawals_db::SingleWithdrawalWitness;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn set(&self, index: u64, withdrawal: Withdrawal) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::Set {
                sender: oneshot_sender,
                index,
                withdrawal,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get(&self, index: u64) -> Option<Withdrawal> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::Get {
                sender: oneshot_sender,
                index,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get_root(&self) -> Option<U256> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetRoot {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn get_witness(&self, index: u64) -> Option<SingleWithdrawalWitness> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                index,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
