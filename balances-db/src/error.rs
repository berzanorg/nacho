use nacho_dynamic_list::DynamicListError;
use nacho_merkle_tree::MerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BalancesDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] MerkleTreeError),
    #[error("Balance doesn't exist.")]
    BalanceDoesntExist,
    #[error("Balance already exists.")]
    BalanceAlreadyExists,
}
