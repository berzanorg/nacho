use nacho_dynamic_list::DynamicListError;
use nacho_dynamic_merkle_tree::DynamicMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BalancesDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] DynamicMerkleTreeError),
    #[error("Balance doesn't exist.")]
    BalanceDoesntExist,
    #[error("Balance already exists.")]
    BalanceAlreadyExists,
}
