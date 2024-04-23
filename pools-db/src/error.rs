use nacho_dynamic_list::DynamicListError;
use nacho_dynamic_merkle_tree::DynamicMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoolsDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] DynamicMerkleTreeError),
    #[error("Pool doesn't exist.")]
    PoolDoesntExist,
    #[error("Pool already exists.")]
    PoolAlreadyExists,
}
