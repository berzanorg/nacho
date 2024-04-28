use thiserror::Error;

/// The errors that can occur during spawning and interacting with Node.js processes.
#[derive(Error, Debug)]
pub enum JsProcessError {
    #[error("data store disconnected")]
    Io(#[from] std::io::Error),
    #[error("stdout couldn't be taken from the js process")]
    Stdout,
    #[error("stdin couldn't be taken from the js process")]
    Stdin,
}
