use nacho_data_structures::U256;
use nacho_pools_db::SinglePoolWitness;
use tokio::sync::oneshot;

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;
/// The alias that represents the type of AMM liquidity points.
type LiquidityPoints = U256;

pub enum Request {
    GetPool {
        sender: oneshot::Sender<Option<(TokenAmount, TokenAmount, LiquidityPoints)>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
    },
    GetPools {
        sender: oneshot::Sender<
            Option<Vec<(TokenId, TokenId, TokenAmount, TokenAmount, LiquidityPoints)>>,
        >,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SinglePoolWitness>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SinglePoolWitness>>,
    },
    PushPool {
        sender: oneshot::Sender<Option<()>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        base_token_amount: TokenAmount,
        quote_token_amount: TokenAmount,
        total_liqudity_points: LiquidityPoints,
    },
    UpdatePool {
        sender: oneshot::Sender<Option<()>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        base_token_amount: TokenAmount,
        quote_token_amount: TokenAmount,
        total_liqudity_points: LiquidityPoints,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        base_token_amount: TokenAmount,
        quote_token_amount: TokenAmount,
        total_liqudity_points: LiquidityPoints,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        base_token_id: TokenId,
        quote_token_id: TokenId,
        base_token_amount: TokenAmount,
        quote_token_amount: TokenAmount,
        total_liqudity_points: LiquidityPoints,
    },
}
