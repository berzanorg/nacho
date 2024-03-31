use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{Address, Burn, U256};
use tokio::sync::oneshot;

pub enum Request {
    GetBurn {
        sender: oneshot::Sender<Option<Burn>>,
        burner: Address,
        token_id: U256,
    },
    GetBurns {
        sender: oneshot::Sender<Option<Vec<Burn>>>,
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
        burn: Burn,
    },
    UpdateBurn {
        sender: oneshot::Sender<Option<()>>,
        burn: Burn,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        burn: Burn,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        burn: Burn,
    },
}
