use nacho_data_structures::U256;
use nacho_withdrawals_db::SingleWithdrawalWitness;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_witness(&self, burn_id: u64) -> Option<SingleWithdrawalWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                burn_id,
            })
            .await
            .ok()?;

        let single_witness = oneshor_receiver.await.ok()?;

        single_witness
    }

    pub async fn set_leaf(&self, burn_id: u64, value: U256) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::SetLeaf {
                sender: oneshot_sender,
                burn_id,
                value,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }
}
