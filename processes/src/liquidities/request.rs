use nacho_data_structures::{Address, Liquidity, U256};
use nacho_liquidities_db::SingleLiquidityWitness;
use tokio::sync::oneshot;

pub enum Request {
    GetLiquidity {
        sender: oneshot::Sender<Option<Liquidity>>,
        provider: Address,
        base_token_id: U256,
        quote_token_id: U256,
    },
    GetLiquidities {
        sender: oneshot::Sender<Option<Vec<Liquidity>>>,
        provider: Address,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SingleLiquidityWitness>>,
        provider: Address,
        base_token_id: U256,
        quote_token_id: U256,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SingleLiquidityWitness>>,
    },
    PushLiquidity {
        sender: oneshot::Sender<Option<()>>,
        liquidity: Liquidity,
    },
    UpdateLiquidity {
        sender: oneshot::Sender<Option<()>>,
        liquidity: Liquidity,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        liquidity: Liquidity,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        liquidity: Liquidity,
    },
    GetRoot {
        sender: oneshot::Sender<Option<U256>>,
    },
}
