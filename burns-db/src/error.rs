use nacho_dynamic_list::DynamicListError;
use nacho_merkle_tree::MerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BurnsDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] MerkleTreeError),
    #[error("Burn doesn't exist.")]
    BurnDoesntExist,
    #[error("Burn already exists.")]
    BurnAlreadyExists,
}
