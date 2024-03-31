use nacho_data_structures::{Pool, U256};
use nacho_pools_db::SinglePoolWitness;
use tokio::sync::oneshot;

pub enum Request {
    GetPool {
        sender: oneshot::Sender<Option<Pool>>,
        base_token_id: U256,
        quote_token_id: U256,
    },
    GetPools {
        sender: oneshot::Sender<Option<Vec<Pool>>>,
    },
    GetWitness {
        sender: oneshot::Sender<Option<SinglePoolWitness>>,
        base_token_id: U256,
        quote_token_id: U256,
    },
    GetNewWitness {
        sender: oneshot::Sender<Option<SinglePoolWitness>>,
    },
    PushPool {
        sender: oneshot::Sender<Option<()>>,
        pool: Pool,
    },
    UpdatePool {
        sender: oneshot::Sender<Option<()>>,
        pool: Pool,
    },
    PushLeaf {
        sender: oneshot::Sender<Option<()>>,
        pool: Pool,
    },
    UpdateLeaf {
        sender: oneshot::Sender<Option<()>>,
        pool: Pool,
    },
}
