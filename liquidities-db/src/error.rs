use nacho_dynamic_list::DynamicListError;
use nacho_dynamic_merkle_tree::DynamicMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LiquiditiesDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    MerkleTree(#[from] DynamicMerkleTreeError),
    #[error("Liquidity doesn't exist.")]
    LiquidityDoesntExist,
    #[error("Liquidity already exists.")]
    LiquidityAlreadyExists,
}
