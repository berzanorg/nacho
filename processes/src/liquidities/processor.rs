use nacho_data_structures::{Address, Liquidity, U256};
use nacho_liquidities_db::SingleLiquidityWitness;
use tokio::sync::{mpsc, oneshot};

use super::Request;

#[derive(Clone, Copy, Debug)]
pub struct Processor {
    pub(crate) sender: &'static mpsc::Sender<Request>,
}

impl Processor {
    pub async fn get_liquidity(
        &self,
        provider: Address,
        base_token_id: U256,
        quote_token_id: U256,
    ) -> Option<Liquidity> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetLiquidity {
                sender: oneshot_sender,
                provider,
                base_token_id,
                quote_token_id,
            })
            .await
            .ok()?;

        let liquidity_points = oneshot_receiver.await.ok()?;

        liquidity_points
    }

    pub async fn get_liquidities(&self, provider: Address) -> Option<Vec<Liquidity>> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetLiquidities {
                sender: oneshot_sender,
                provider,
            })
            .await
            .ok()?;

        let liquidities = oneshot_receiver.await.ok()?;

        liquidities
    }

    pub async fn get_witness(
        &self,
        provider: Address,
        base_token_id: U256,
        quote_token_id: U256,
    ) -> Option<SingleLiquidityWitness> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::GetWitness {
                sender: oneshot_sender,
                provider,
                base_token_id,
                quote_token_id,
            })
            .await
            .ok()?;

        let single_witness = oneshot_receiver.await.ok()?;

        single_witness
    }

    pub async fn get_new_witness(&self) -> Option<SingleLiquidityWitness> {
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

    pub async fn push_liquidity(&self, liquidity: Liquidity) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLiquidity {
                sender: oneshot_sender,
                liquidity,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_liquidity(&self, liquidity: Liquidity) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLiquidity {
                sender: oneshot_sender,
                liquidity,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn push_leaf(&self, liquidity: Liquidity) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::PushLeaf {
                sender: oneshot_sender,
                liquidity,
            })
            .await
            .ok()?;

        let result = oneshot_receiver.await.ok()?;

        result
    }

    pub async fn update_leaf(&self, liquidity: Liquidity) -> Option<()> {
        let (oneshot_sender, oneshot_receiver) = oneshot::channel();

        self.sender
            .send(Request::UpdateLeaf {
                sender: oneshot_sender,
                liquidity,
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
}
