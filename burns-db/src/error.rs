use nacho_dynamic_list::DynamicListError;
use nacho_dynamic_merkle_tree::DynamicMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BurnsDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] DynamicMerkleTreeError),
    #[error("Burn doesn't exist.")]
    BurnDoesntExist,
    #[error("Burn already exists.")]
    BurnAlreadyExists,
}
