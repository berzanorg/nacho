use ark_ff::PrimeField;
use data_structures::{Field, TxStatus};
use http_body_util::Full;
use hyper::{body::Bytes, Response};

/// The alias that represents the type of token IDs.
type TokenId = Field;
/// The alias that represents the type of token amounts.
type TokenAmount = u64;
/// The alias that represents the type of AMM liquidity points.
type LiquidityPoints = u128;

/// The enum that represents RPC responses.
///
/// It can be deserialized into bytes.
pub enum RpcResponse {
    /// The RPC response that is sent when a request causes an error.
    Mistake(),
    /// The RPC response that is sent when a transaction's status is requested.
    TxStatus(TxStatus),
    /// The RPC response that is sent when the balances of an address are requested.
    Balances(Vec<(TokenId, TokenAmount)>),
    /// The RPC response that is sent when all the AMM pools are requested.
    GetPools(Vec<(TokenId, TokenId, TokenAmount, TokenAmount)>),
    /// The RPC response that is sent when an AMM pool is requested.
    GetPool((TokenId, TokenId, TokenAmount, TokenAmount)),
    /// The RPC response that is sent when the liquidities of an address are requested.
    GetLiquidities(Vec<(TokenId, TokenId, LiquidityPoints)>),
    /// The RPC response that is sent when the burns of an address are requested.
    GetBurns(Vec<(TokenId, TokenAmount)>),
    GetStateWitness(),
    GetBurnWitness(),
    GetDepositWitness(),
    // TODO: add transaction methods
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
            RpcResponse::Mistake() => {
                let mut buf = Vec::with_capacity(1);
                buf.push(0);

                buf
            }
            RpcResponse::TxStatus(tx_status) => {
                let mut buf = Vec::with_capacity(1 + 1);
                buf.push(1);

                buf.push(tx_status as u8);

                buf
            }
            RpcResponse::Balances(balances) => {
                let mut buf = Vec::with_capacity(1 + balances.len() * 40);
                buf.push(2);

                for (token_id, token_amount) in balances {
                    let token_id = token_id.into_repr();
                    buf.copy_from_slice(&token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&token_amount.to_le_bytes());
                }

                buf
            }
            RpcResponse::GetPools(pools) => {
                let mut buf = Vec::with_capacity(1 + pools.len() * 80);
                buf.push(3);

                for (base_token_id, quote_token_id, base_token_amount, quote_token_amount) in pools
                {
                    let base_token_id = base_token_id.into_repr();
                    let quote_token_id = quote_token_id.into_repr();
                    buf.copy_from_slice(&base_token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&base_token_amount.to_le_bytes());
                    buf.copy_from_slice(&quote_token_amount.to_le_bytes());
                }

                buf
            }
            RpcResponse::GetPool((
                base_token_id,
                quote_token_id,
                base_token_amount,
                quote_token_amount,
            )) => {
                let mut buf = Vec::with_capacity(1 + 80);
                buf.push(4);

                let base_token_id = base_token_id.into_repr();
                let quote_token_id = quote_token_id.into_repr();
                buf.copy_from_slice(&base_token_id.0[0].to_le_bytes());
                buf.copy_from_slice(&base_token_id.0[1].to_le_bytes());
                buf.copy_from_slice(&base_token_id.0[2].to_le_bytes());
                buf.copy_from_slice(&base_token_id.0[3].to_le_bytes());
                buf.copy_from_slice(&quote_token_id.0[0].to_le_bytes());
                buf.copy_from_slice(&quote_token_id.0[1].to_le_bytes());
                buf.copy_from_slice(&quote_token_id.0[2].to_le_bytes());
                buf.copy_from_slice(&quote_token_id.0[3].to_le_bytes());
                buf.copy_from_slice(&base_token_amount.to_le_bytes());
                buf.copy_from_slice(&quote_token_amount.to_le_bytes());

                buf
            }
            RpcResponse::GetLiquidities(liquidities) => {
                let mut buf = Vec::with_capacity(1 + liquidities.len() * 80);
                buf.push(5);

                for (base_token_id, quote_token_id, liquidity_points) in liquidities {
                    let base_token_id = base_token_id.into_repr();
                    let quote_token_id = quote_token_id.into_repr();
                    buf.copy_from_slice(&base_token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&base_token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&quote_token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&liquidity_points.to_le_bytes());
                }

                buf
            }
            RpcResponse::GetBurns(burns) => {
                let mut buf = Vec::with_capacity(1 + burns.len() * 40);
                buf.push(6);

                for (token_id, token_amount) in burns {
                    let token_id = token_id.into_repr();
                    buf.copy_from_slice(&token_id.0[0].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[1].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[2].to_le_bytes());
                    buf.copy_from_slice(&token_id.0[3].to_le_bytes());
                    buf.copy_from_slice(&token_amount.to_le_bytes());
                }

                buf
            }
            RpcResponse::GetStateWitness() => {
                todo!()
            }
            RpcResponse::GetBurnWitness() => {
                todo!()
            }
            RpcResponse::GetDepositWitness() => {
                todo!()
            }
        }
    }
}
