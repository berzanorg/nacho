use nacho_data_structures::{Pool, U256};
use nacho_pools_db::SinglePoolWitness;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_pool(&self, base_token_id: U256, quote_token_id: U256) -> Option<Pool> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetPool {
                sender: oneshot_sender,
                base_token_id,
                quote_token_id,
            })
            .await
            .ok()?;

        let pool = oneshot_receiver.await.ok()?;

        pool
    }

    pub async fn get_pools(&self) -> Option<Vec<Pool>> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetPools {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let pools = oneshot_receiver.await.ok()?;

        pools
    }

    pub async fn get_witness(
        &self,
        base_token_id: U256,
        quote_token_id: U256,
    ) -> Option<SinglePoolWitness> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                base_token_id,
                quote_token_id,
            })
            .await
            .ok()?;

        let single_witness = oneshot_receiver.await.ok()?;

        single_witness
    }

    pub async fn get_new_witness(&self) -> Option<SinglePoolWitness> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetNewWitness {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let new_witness = oneshot_receiver.await.ok()?;

        new_witness
    }

    pub async fn push_pool(&self, pool: Pool) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushPool {
                sender: oneshot_sender,
                pool,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_pool(&self, pool: Pool) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdatePool {
                sender: oneshot_sender,
                pool,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn push_leaf(&self, pool: Pool) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLeaf {
                sender: oneshot_sender,
                pool,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_leaf(&self, pool: Pool) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLeaf {
                sender: oneshot_sender,
                pool,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
