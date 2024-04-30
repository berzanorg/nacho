use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{Address, Burn, U256};
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_burn(&self, burner: Address, token_id: U256) -> Option<Burn> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetBurn {
                sender: oneshot_sender,
                burner,
                token_id,
            })
            .await
            .ok()?;

        let burn = oneshot_receiver.await.ok()?;

        burn
    }

    pub async fn get_burns(&self, burner: Address) -> Option<Vec<Burn>> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetBurns {
                sender: oneshot_sender,
                burner,
            })
            .await
            .ok()?;

        let burns = oneshot_receiver.await.ok()?;

        burns
    }

    pub async fn get_witness(
        &self,
        burner: Address,
        token_id: U256,
    ) -> Option<(SingleBurnWitness, u64)> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                burner,
                token_id,
            })
            .await
            .ok()?;

        let single_witness = oneshot_receiver.await.ok()?;

        single_witness
    }

    pub async fn get_new_witness(&self) -> Option<SingleBurnWitness> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetNewWitness {
                sender: oneshot_sender,
            })
            .await
            .ok()?;

        let single_witness = oneshot_receiver.await.ok()?;

        single_witness
    }

    pub async fn push_burn(&self, burn: Burn) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushBurn {
                sender: oneshot_sender,
                burn,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_burn(&self, burn: Burn) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateBurn {
                sender: oneshot_sender,
                burn,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn push_leaf(&self, burn: Burn) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLeaf {
                sender: oneshot_sender,
                burn,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_leaf(&self, burn: Burn) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLeaf {
                sender: oneshot_sender,
                burn,
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

    pub async fn get_index(&self, burner: Address, token_id: U256) -> Option<u64> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetIndex {
                sender: oneshot_sender,
                burner,
                token_id,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }
}
