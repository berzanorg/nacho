use nacho_dynamic_list::DynamicListError;
use nacho_merkle_tree::MerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiquiditiesDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] MerkleTreeError),
    #[error("Liquidity doesn't exist.")]
    LiquidityDoesntExist,
    #[error("Liquidity already exists.")]
    LiquidityAlreadyExists,
}
