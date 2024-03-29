use nacho_data_structures::{Address, FromBytes, Signature, U256};

/// The enum that represents RPC methods.
///
/// It can be serialized from bytes received over the network.
///
/// Instances of this type are created inside `start_rpc_server` under the hood.
///
/// It should only be used inside RPC method handler which is a parameter of `start_rpc_server` function.
pub enum RpcMethod {
    /// Represents unknown methods which can't be parsed.
    Unknown,

    /// Requests the total transaction count.
    GetTotalTxCount,

    /// Requests a transaction's status.
    GetTxStatus { tx_id: u64 },

    /// Requests all the balances of a user.
    GetBalances { address: Address },

    /// Requests all the AMM pools.
    GetPools,

    /// Requests all the AMM liquidites of a user.
    GetLiquidities { address: Address },

    /// Requests all the token burns of a user.
    GetBurns { address: Address },

    /// Requests witnesses required to use the bridge.
    GetBridgeWitnesses { burn_id: u64 },

    /// Requests to burn tokens.
    BurnTokens {
        address: Address,
        signature: Signature,
        token_id: U256,
        token_amount: u64,
    },

    /// Requests to create an AMM pool.
    CreatePool {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount: u64,
    },

    /// Requests to provide liquidity for an AMM pool.
    ProvideLiquidity {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount_limit: u64,
    },

    /// Requests to remove liquidity from an AMM pool.
    RemoveLiquidity {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount_limit: u64,
        quote_token_amount_limit: u64,
        liquidity_point_amount: U256,
    },

    /// Requests to buy the base token of an AMM pool.
    BuyTokens {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount: u64,
        quote_token_amount_limit: u64,
    },

    /// Requests to sell the base token of an AMM pool.
    SellTokens {
        address: Address,
        signature: Signature,
        base_token_id: U256,
        quote_token_id: U256,
        base_token_amount_limit: u64,
        quote_token_amount: u64,
    },
}

impl RpcMethod {
    /// The size of an RPC method over the network.
    pub const SIZE_IN_BYTES: usize = 264;
}

impl From<[u8; RpcMethod::SIZE_IN_BYTES]> for RpcMethod {
    fn from(bytes: [u8; RpcMethod::SIZE_IN_BYTES]) -> Self {
        match bytes[0] {
            1 => RpcMethod::GetTotalTxCount,

            2 => RpcMethod::GetTxStatus {
                tx_id: u64::from_bytes(bytes[1..9].try_into().unwrap()),
            },

            3 => RpcMethod::GetBalances {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
            },

            4 => RpcMethod::GetPools,

            5 => RpcMethod::GetLiquidities {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
            },

            6 => RpcMethod::GetBurns {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
            },

            7 => RpcMethod::GetBridgeWitnesses {
                burn_id: u64::from_bytes(bytes[1..9].try_into().unwrap()),
            },

            8 => RpcMethod::BurnTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                token_amount: u64::from_bytes(bytes[184..192].try_into().unwrap()),
            },

            9 => RpcMethod::CreatePool {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[184..216].try_into().unwrap()),
                base_token_amount: u64::from_bytes(bytes[216..224].try_into().unwrap()),
                quote_token_amount: u64::from_bytes(bytes[224..232].try_into().unwrap()),
            },

            10 => RpcMethod::ProvideLiquidity {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[184..216].try_into().unwrap()),
                base_token_amount: u64::from_bytes(bytes[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_bytes(bytes[224..232].try_into().unwrap()),
            },

            11 => RpcMethod::RemoveLiquidity {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[184..216].try_into().unwrap()),
                base_token_amount_limit: u64::from_bytes(bytes[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_bytes(bytes[224..232].try_into().unwrap()),
                liquidity_point_amount: U256::from_bytes(bytes[232..264].try_into().unwrap()),
            },

            12 => RpcMethod::BuyTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[184..216].try_into().unwrap()),
                base_token_amount: u64::from_bytes(bytes[216..224].try_into().unwrap()),
                quote_token_amount_limit: u64::from_bytes(bytes[224..232].try_into().unwrap()),
            },

            13 => RpcMethod::SellTokens {
                address: Address::from_bytes(bytes[1..56].try_into().unwrap()),
                signature: Signature::from_bytes(bytes[56..152].try_into().unwrap()),
                base_token_id: U256::from_bytes(bytes[152..184].try_into().unwrap()),
                quote_token_id: U256::from_bytes(bytes[184..216].try_into().unwrap()),
                base_token_amount_limit: u64::from_bytes(bytes[216..224].try_into().unwrap()),
                quote_token_amount: u64::from_bytes(bytes[224..232].try_into().unwrap()),
            },

            _ => RpcMethod::Unknown,
        }
    }
}
