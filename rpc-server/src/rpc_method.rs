use crate::RpcMethodError;
use ark_ff::FromBytes;
use data_structures::{Field, PublicKey};
use RpcMethod::*;

/// The size of an RPC method over the network.
pub(crate) const RPC_METHOD_SIZE_IN_BYTES: usize = 65;

/// The enum that represents RPC methods.
///
/// It can be serialized from bytes received over the network.
///
/// Instances of this type are created inside `start_rpc_server` under the hood.
///
/// It should only be used inside RPC method handler which is a parameter of `start_rpc_server` function.
///
pub enum RpcMethod {
    /// The method that requests a transaction's index.
    GetTxStatus {
        tx_index: u64,
    },
    /// The method that requests the balances of an address.
    GetBalances {
        address: PublicKey,
    },
    /// The method that requests all the AMM pools.
    GetPools {},
    /// The method that requests an AMM pool.
    GetPool {
        base_token_id: Field,
        quote_token_id: Field,
    },
    /// The method that requests the AMM liquidities of an address.
    GetLiquidities {
        address: PublicKey,
    },
    /// The method that requests the burns of an address.
    GetBurns {
        address: PublicKey,
    },
    GetStateWitness {
        index: u64,
    },
    GetBurnWitness {
        index: u64,
    },
    GetDepositWitness {
        index: u64,
    },
    // TODO: add transaction methods
}

impl TryFrom<[u8; RPC_METHOD_SIZE_IN_BYTES]> for RpcMethod {
    type Error = RpcMethodError;

    fn try_from(bytes: [u8; RPC_METHOD_SIZE_IN_BYTES]) -> Result<Self, Self::Error> {
        let res = match bytes[0] {
            1 => GetTxStatus {
                tx_index: u64::from_le_bytes(bytes[1..9].try_into().unwrap()),
            },
            2 => GetBalances {
                address: PublicKey::from_bytes(&bytes[1..34])?,
            },
            3 => GetPools {},
            4 => GetPool {
                base_token_id: Field::read(&bytes[1..33])?,
                quote_token_id: Field::read(&bytes[33..65])?,
            },
            5 => GetLiquidities {
                address: PublicKey::from_bytes(&bytes[1..34])?,
            },
            6 => GetBurns {
                address: PublicKey::from_bytes(&bytes[1..34])?,
            },
            7 => GetStateWitness {
                index: u64::from_le_bytes(bytes[1..9].try_into().unwrap()),
            },
            8 => GetBurnWitness {
                index: u64::from_le_bytes(bytes[1..9].try_into().unwrap()),
            },
            9 => GetDepositWitness {
                index: u64::from_le_bytes(bytes[1..9].try_into().unwrap()),
            },
            _ => return Err(RpcMethodError::UnknownMethod),
        };

        Ok(res)
    }
}
