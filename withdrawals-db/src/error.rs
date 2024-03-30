use thiserror::Error;

#[derive(Error, Debug)]
pub enum WithdrawalsDbError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("Parent directory of Withdrawals DB isn't specified.")]
    ParentDirectoryNotSpecified,
    #[error("Leaf indexes are exceeded.")]
    LeafIndexesExceeded,
    #[error("Index you are trying to acces isn't accessible yet.")]
    UnusableIndex,
}
