use data_structures::{ToBytes, TxStatus, U256};
use http_body_util::Full;
use hyper::{body::Bytes, Response};

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;
/// The alias that represents the type of AMM liquidity points.
type LiquidityPoints = U256;
/// The alias that represents the type of burn IDs.
type BurnId = u64;

/// The enum that represents RPC responses.
///
/// It can be deserialized into bytes.
pub enum RpcResponse {
    /// Used for unknown RPC methods.
    UnknownMethod,
    /// Represents the total transaction count.
    TotalTxCount(u64),
    /// Represents the status of a transaction.
    TxStatus(TxStatus),
    /// Represents the balances of a user.
    Balances(Vec<(TokenId, TokenAmount)>),
    /// Represents all the AMM pools.
    Pools(Vec<(TokenId, TokenId, TokenAmount, TokenAmount, LiquidityPoints)>),
    /// Represents the AMM liquidites of a user.
    Liquidites(Vec<(TokenId, TokenId, LiquidityPoints)>),
    /// Represents the burns of a user.
    Burns(Vec<(TokenId, TokenAmount, BurnId)>),
    /// Represents the witnesses needed to withdraw assets from the bridge.
    BridgeWitnesses((bool, bool)),
    /// Represents a transaction's ID. Used for the RPC methods that modify the state.
    TxId(u64),
}

impl From<RpcResponse> for Result<Response<Full<Bytes>>, String> {
    fn from(value: RpcResponse) -> Self {
        let res: Vec<u8> = value.into();
        Ok(Response::new(Full::from(res)))
    }
}

impl From<RpcResponse> for Vec<u8> {
    fn from(value: RpcResponse) -> Self {
        match value {
            RpcResponse::UnknownMethod => vec![0u8; 1],
            RpcResponse::TotalTxCount(total_tx_count) => {
                let mut bytes = Vec::with_capacity(1 + 8);
                bytes[0] = 1;

                bytes.copy_from_slice(&total_tx_count.to_le_bytes());

                bytes
            }
            RpcResponse::TxStatus(tx_status) => {
                let mut bytes = Vec::with_capacity(1 + 1);
                bytes[0] = 2;

                bytes.push(tx_status as u8);

                bytes
            }
            RpcResponse::Balances(balances) => {
                let mut bytes = Vec::with_capacity(1 + (40 * balances.len()));
                bytes[0] = 3;

                for (token_id, token_amount) in balances {
                    bytes.copy_from_slice(&token_id.to_bytes());
                    bytes.copy_from_slice(&token_amount.to_bytes());
                }

                bytes
            }
            RpcResponse::Pools(pools) => {
                let mut bytes = Vec::with_capacity(1 + (112 * pools.len()));
                bytes[0] = 4;

                for (
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liquidity_points,
                ) in pools
                {
                    bytes.copy_from_slice(&base_token_id.to_bytes());
                    bytes.copy_from_slice(&quote_token_id.to_bytes());
                    bytes.copy_from_slice(&base_token_amount.to_bytes());
                    bytes.copy_from_slice(&quote_token_amount.to_bytes());
                    bytes.copy_from_slice(&total_liquidity_points.to_bytes());
                }

                bytes
            }
            RpcResponse::Liquidites(liqudities) => {
                let mut bytes = Vec::with_capacity(1 + (96 * liqudities.len()));
                bytes[0] = 5;

                for (base_token_id, quote_token_id, liquidity_points) in liqudities {
                    bytes.copy_from_slice(&base_token_id.to_bytes());
                    bytes.copy_from_slice(&quote_token_id.to_bytes());
                    bytes.copy_from_slice(&liquidity_points.to_bytes());
                }

                bytes
            }
            RpcResponse::Burns(burns) => {
                let mut bytes = Vec::with_capacity(1 + (48 * burns.len()));
                bytes[0] = 6;

                for (token_id, token_amount, burn_id) in burns {
                    bytes.copy_from_slice(&token_id.to_bytes());
                    bytes.copy_from_slice(&token_amount.to_bytes());
                    bytes.copy_from_slice(&burn_id.to_bytes());
                }

                bytes
            }
            RpcResponse::BridgeWitnesses(_) => {
                let mut bytes = Vec::with_capacity(1 + 69);
                bytes[0] = 7;

                bytes
            }
            RpcResponse::TxId(tx_id) => {
                let mut bytes = Vec::with_capacity(1 + 8);
                bytes[0] = 8;

                bytes.copy_from_slice(&tx_id.to_bytes());

                bytes
            }
        }
    }
}
