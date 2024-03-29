use nacho_dynamic_list::DynamicListError;
use nacho_merkle_tree::MerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PoolsDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] MerkleTreeError),
    #[error("Pool doesn't exist.")]
    PoolDoesntExist,
    #[error("Pool already exists.")]
    PoolAlreadyExists,
}
