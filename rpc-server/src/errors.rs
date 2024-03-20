use thiserror::Error;

/// The error type that represents the errors that can happen during RPC method serialization.
#[derive(Error, Debug)]
pub enum RpcMethodError {
    #[error("cannot parse public key")]
    MistakenPublicKey(#[from] mina_signer::pubkey::PubKeyError),
    #[error("cannot parse field")]
    MistakenField(#[from] std::io::Error),
    #[error("unknown rpc method")]
    UnknownMethod,
}
