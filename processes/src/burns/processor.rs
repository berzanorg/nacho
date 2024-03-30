use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{Address, U256};
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_burn(&self, burner: Address, token_id: U256) -> Option<u64> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetBurn {
                sender: oneshot_sender,
                burner,
                token_id,
            })
            .await
            .ok()?;

        let burn = oneshor_receiver.await.ok()?;

        burn
    }

    pub async fn get_burns(&self, burner: Address) -> Option<Vec<(U256, u64)>> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetBurns {
                sender: oneshot_sender,
                burner,
            })
            .await
            .ok()?;

        let burns = oneshor_receiver.await.ok()?;

        burns
    }

    pub async fn get_witness(&self, burner: Address, token_id: U256) -> Option<SingleBurnWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                burner,
                token_id,
            })
            .await
            .ok()?;

        let single_witness = oneshor_receiver.await.ok()?;

        single_witness
    }

    pub async fn get_new_witness(&self) -> Option<SingleBurnWitness> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetNewWitness {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let single_witness = oneshor_receiver.await.ok()?;

        single_witness
    }

    pub async fn push_burn(
        &self,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushBurn {
                sender: oneshot_sender,
                burner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn update_burn(
        &self,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateBurn {
                sender: oneshot_sender,
                burner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn push_leaf(
        &self,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLeaf {
                sender: oneshot_sender,
                burner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }

    pub async fn update_leaf(
        &self,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    ) -> Option<()> {
        let (oneshot_sender, oneshor_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLeaf {
                sender: oneshot_sender,
                burner,
                token_id,
                token_amount,
            })
            .await
            .ok()?;

        let result = oneshor_receiver.await.ok()?;

        result
    }
}
