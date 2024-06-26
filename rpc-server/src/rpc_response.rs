use http_body_util::Full;
use hyper::{body::Bytes, Response};
use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{ByteConversion, TxStatus, U256};
use nacho_withdrawals_db::SingleWithdrawalWitness;

/// The alias that represents the type of token IDs.
type TokenId = U256;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;
/// The alias that represents the type of AMM liquidity points.
type LiquidityPoints = U256;

/// The enum that represents RPC responses.
///
/// It can be deserialized into bytes.
pub enum RpcResponse {
    /// Used for unknown RPC methods.
    ClientError,
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
    Burns(Vec<(TokenId, TokenAmount)>),
    /// Represents the witnesses needed to withdraw assets from the bridge.
    BridgeWitnesses(SingleBurnWitness, SingleWithdrawalWitness),
    /// Represents a transaction's ID. Used for the RPC methods that modify the state.
    TxId(u64),
    /// Used for unknown RPC methods.
    ServerError,
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
            RpcResponse::ClientError => vec![0u8; 1],
            RpcResponse::TotalTxCount(total_tx_count) => {
                let mut bytes = Vec::with_capacity(1 + 8);
                bytes.push(1);

                bytes.extend_from_slice(&total_tx_count.to_le_bytes());

                bytes
            }
            RpcResponse::TxStatus(tx_status) => {
                let mut bytes = Vec::with_capacity(1 + 1);
                bytes.push(2);

                bytes.push(tx_status as u8);

                bytes
            }
            RpcResponse::Balances(balances) => {
                let mut bytes = Vec::with_capacity(1 + (40 * balances.len()));
                bytes.push(3);

                for (token_id, token_amount) in balances {
                    bytes.extend_from_slice(&token_id.to_bytes());
                    bytes.extend_from_slice(&token_amount.to_bytes());
                }

                bytes
            }
            RpcResponse::Pools(pools) => {
                let mut bytes = Vec::with_capacity(1 + (112 * pools.len()));
                bytes.push(4);

                for (
                    base_token_id,
                    quote_token_id,
                    base_token_amount,
                    quote_token_amount,
                    total_liquidity_points,
                ) in pools
                {
                    bytes.extend_from_slice(&base_token_id.to_bytes());
                    bytes.extend_from_slice(&quote_token_id.to_bytes());
                    bytes.extend_from_slice(&base_token_amount.to_bytes());
                    bytes.extend_from_slice(&quote_token_amount.to_bytes());
                    bytes.extend_from_slice(&total_liquidity_points.to_bytes());
                }

                bytes
            }
            RpcResponse::Liquidites(liqudities) => {
                let mut bytes = Vec::with_capacity(1 + (96 * liqudities.len()));
                bytes.push(5);

                for (base_token_id, quote_token_id, liquidity_points) in liqudities {
                    bytes.extend_from_slice(&base_token_id.to_bytes());
                    bytes.extend_from_slice(&quote_token_id.to_bytes());
                    bytes.extend_from_slice(&liquidity_points.to_bytes());
                }

                bytes
            }
            RpcResponse::Burns(burns) => {
                let mut bytes = Vec::with_capacity(1 + (40 * burns.len()));
                bytes.push(6);

                for (token_id, token_amount) in burns {
                    bytes.extend_from_slice(&token_id.to_bytes());
                    bytes.extend_from_slice(&token_amount.to_bytes());
                }

                bytes
            }
            RpcResponse::BridgeWitnesses(burn_witness, withdrawal_witness) => {
                let mut bytes = Vec::with_capacity(1 + 627 + 594);
                bytes.push(7);

                bytes.extend_from_slice(&burn_witness.to_bytes());
                bytes.extend_from_slice(&withdrawal_witness.to_bytes());

                bytes
            }
            RpcResponse::TxId(tx_id) => {
                let mut bytes = Vec::with_capacity(1 + 8);
                bytes.push(8);

                bytes.extend_from_slice(&tx_id.to_bytes());

                bytes
            }
            RpcResponse::ServerError => vec![9u8; 1],
        }
    }
}
