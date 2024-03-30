use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{Address, U256};
use tokio::sync::oneshot;

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;

pub enum Request {
    GetBurn {
        sender: oneshot::Sender<Option<TokenAmount>>,
        burner: Address,
        token_id: U256,
    },
    GetBurns {
        sender: oneshot::Sender<Option<Vec<(TokenId, TokenAmount)>>>,
        burner: Address,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SingleBurnWitness>>,
        burner: Address,
        token_id: U256,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SingleBurnWitness>>,
    },
    PushBurn {
        sender: oneshot::Sender<Option<()>>,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    },
    UpdateBurn {
        sender: oneshot::Sender<Option<()>>,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        burner: Address,
        token_id: U256,
        token_amount: u64,
    },
}
