use nacho_data_structures::{Address, U256};
use nacho_liquidities_db::SingleLiquidityWitness;
use tokio::sync::oneshot;

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of AMM liquidity points.
type LiquidityPoints = U256;

pub enum Request {
    GetLiquidityPoints {
        sender: oneshot::Sender<Option<LiquidityPoints>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
    },
    GetLiquidities {
        sender: oneshot::Sender<Option<Vec<(TokenId, TokenId, LiquidityPoints)>>>,
        provider: Address,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SingleLiquidityWitness>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SingleLiquidityWitness>>,
    },
    PushLiquidity {
        sender: oneshot::Sender<Option<()>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        points: LiquidityPoints,
    },
    UpdateLiquidity {
        sender: oneshot::Sender<Option<()>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        points: LiquidityPoints,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        points: LiquidityPoints,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        provider: Address,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        points: LiquidityPoints,
    },
}
