use nacho_static_list::StaticListError;
use nacho_static_merkle_tree::StaticMerkleTreeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WithdrawalsDbError {
    #[error(transparent)]
    StaticList(#[from] StaticListError),
    #[error(transparent)]
    StaticMerkleTree(#[from] StaticMerkleTreeError),
}
