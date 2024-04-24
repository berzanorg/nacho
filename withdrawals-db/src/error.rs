use nacho_dynamic_list::DynamicListError;
use nacho_static_merkle_tree::StaticMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WithdrawalsDbError {
    #[error(transparent)]
    DynamicList(#[from] DynamicListError),
    #[error(transparent)]
    StaticMerkleTree(#[from] StaticMerkleTreeError),
}
