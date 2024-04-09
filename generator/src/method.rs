use nacho_balances_db::{DoubleBalanceWitness, SingleBalanceWitness};
use nacho_burns_db::SingleBurnWitness;
use nacho_data_structures::{Address, Signature, StateRoots, U256};
use nacho_liquidities_db::SingleLiquidityWitness;
use nacho_pools_db::SinglePoolWitness;

/// The enum that represents method types.
#[derive(Clone, Debug)]
pub enum Method {
    CreateGenesis {
        state_roots: StateRoots,
    },
    DepositTokens {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_balance_witness: SingleBalanceWitness,
        current_deposits_merkle_list_hash: U256,
        expected_deposits_merkle_list_hash: U256,
        user_address: Address,
        token_id: U256,
        user_deposit_token_amount: u64,
        user_balance_token_amount: u64,
        is_users_first_deposit: bool,
    },
    BurnTokens {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_balance_witness: SingleBalanceWitness,
        single_burn_witness: SingleBurnWitness,
        user_address: Address,
        token_id: U256,
        user_burn_token_amount: u64,
        user_balance_token_amount: u64,
        amount_to_burn: u64,
        user_signature: Signature,
    },
    CreatePool {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_pool_witness: SinglePoolWitness,
        single_liquidity_witness: SingleLiquidityWitness,
        double_balance_witness: DoubleBalanceWitness,
        base_token_id: U256,
        quote_token_id: U256,
        user_address: Address,
        user_liquidity_base_token_amount: u64,
        user_liquidity_quote_token_amount: u64,
        user_balance_base_token_amount: u64,
        user_balance_quote_token_amount: u64,
        user_signature: Signature,
    },
    ProvideLiquidity {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_pool_witness: SinglePoolWitness,
        single_liquidity_witness: SingleLiquidityWitness,
        double_balance_witness: DoubleBalanceWitness,
        base_token_id: U256,
        quote_token_id: U256,
        user_address: Address,
        user_liquidity_points: U256,
        user_balance_base_token_amount: u64,
        user_balance_quote_token_amount: u64,
        pool_base_token_amount: u64,
        pool_quote_token_amount: u64,
        pool_total_liquidity_points: U256,
        user_base_token_amount_to_provide: u64,
        user_quote_token_amount_limit_to_provide: u64,
        is_first_providing: bool,
        user_signature: Signature,
    },
    RemoveLiquidity {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_pool_witness: SinglePoolWitness,
        single_liquidity_witness: SingleLiquidityWitness,
        double_balance_witness: DoubleBalanceWitness,
        base_token_id: U256,
        quote_token_id: U256,
        user_address: Address,
        user_liquidity_points: U256,
        user_balance_base_token_amount: u64,
        user_balance_quote_token_amount: u64,
        pool_base_token_amount: u64,
        pool_quote_token_amount: u64,
        pool_total_liquidity_points: U256,
        user_liquidity_points_to_remove: U256,
        user_base_token_amount_limit_to_remove: u64,
        user_quote_token_amount_limit_to_remove: u64,
        user_signature: Signature,
    },
    BuyTokens {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_pool_witness: SinglePoolWitness,
        double_balance_witness: DoubleBalanceWitness,
        user_address: Address,
        base_token_id: U256,
        quote_token_id: U256,
        user_balance_base_token_amount: u64,
        user_balance_quote_token_amount: u64,
        pool_base_token_amount: u64,
        pool_quote_token_amount: u64,
        pool_total_liquidity_points: U256,
        user_base_token_amount_to_swap: u64,
        user_quote_token_amount_limit_to_swap: u64,
        user_signature: Signature,
    },
    SellTokens {
        state_roots: StateRoots,
        earlier_proof_index: u64,
        single_pool_witness: SinglePoolWitness,
        double_balance_witness: DoubleBalanceWitness,
        user_address: Address,
        base_token_id: U256,
        quote_token_id: U256,
        user_balance_base_token_amount: u64,
        user_balance_quote_token_amount: u64,
        pool_base_token_amount: u64,
        pool_quote_token_amount: u64,
        pool_total_liquidity_points: U256,
        user_base_token_amount_limit_to_swap: u64,
        user_quote_token_amount_to_swap: u64,
        user_signature: Signature,
    },
}

// impl ToBytes for Method {
//     type Bytes = [u8; 3328];

//     fn to_bytes(&self) -> Self::Bytes {
//         let mut buf = [0; 3328];

//         match self {
//             Method::CreateGenesis { state_roots } => {
//                 buf[0] = 0;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//             }
//             Method::DepositTokens {
//                 state_roots,
//                 earlier_proof_index,
//                 single_balance_witness,
//                 current_deposits_merkle_list_hash,
//                 expected_deposits_merkle_list_hash,
//                 user_address,
//                 token_id,
//                 user_deposit_token_amount,
//                 user_balance_token_amount,
//                 is_users_first_deposit,
//             } => {
//                 buf[0] = 1;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..863].copy_from_slice(&single_balance_witness.to_bytes());
//                 buf[863..895].copy_from_slice(&current_deposits_merkle_list_hash.to_bytes());
//                 buf[895..927].copy_from_slice(&expected_deposits_merkle_list_hash.to_bytes());
//                 buf[927..982].copy_from_slice(&user_address.to_bytes());
//                 buf[982..1014].copy_from_slice(&token_id.to_bytes());
//                 buf[1014..1022].copy_from_slice(&user_deposit_token_amount.to_bytes());
//                 buf[1022..1030].copy_from_slice(&user_balance_token_amount.to_bytes());
//                 buf[1030] = *is_users_first_deposit as u8;
//             }
//             Method::BurnTokens {
//                 state_roots,
//                 earlier_proof_index,
//                 single_balance_witness,
//                 single_burn_witness,
//                 user_address,
//                 token_id,
//                 user_burn_token_amount,
//                 user_balance_token_amount,
//                 amount_to_burn,
//                 user_signature,
//             } => {
//                 buf[0] = 2;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..863].copy_from_slice(&single_balance_witness.to_bytes());
//                 buf[863..1490].copy_from_slice(&single_burn_witness.to_bytes());
//                 buf[1490..1545].copy_from_slice(&user_address.to_bytes());
//                 buf[1545..1577].copy_from_slice(&token_id.to_bytes());
//                 buf[1577..1585].copy_from_slice(&user_burn_token_amount.to_bytes());
//                 buf[1585..1593].copy_from_slice(&user_balance_token_amount.to_bytes());
//                 buf[1593..1601].copy_from_slice(&amount_to_burn.to_bytes());
//                 buf[1601..1697].copy_from_slice(&user_signature.to_bytes());
//             }
//             Method::CreatePool {
//                 state_roots,
//                 earlier_proof_index,
//                 single_pool_witness,
//                 single_liquidity_witness,
//                 double_balance_witness,
//                 base_token_id,
//                 quote_token_id,
//                 user_address,
//                 user_liquidity_base_token_amount,
//                 user_liquidity_quote_token_amount,
//                 user_balance_base_token_amount,
//                 user_balance_quote_token_amount,
//                 user_signature,
//             } => {
//                 buf[0] = 3;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..797].copy_from_slice(&single_pool_witness.to_bytes());
//                 buf[797..1490].copy_from_slice(&single_liquidity_witness.to_bytes());
//                 buf[1490..2964].copy_from_slice(&double_balance_witness.to_bytes());
//                 buf[2964..2996].copy_from_slice(&base_token_id.to_bytes());
//                 buf[2996..3028].copy_from_slice(&quote_token_id.to_bytes());
//                 buf[3028..3083].copy_from_slice(&user_address.to_bytes());
//                 buf[3083..3091].copy_from_slice(&user_liquidity_base_token_amount.to_bytes());
//                 buf[3091..3099].copy_from_slice(&user_liquidity_quote_token_amount.to_bytes());
//                 buf[3099..3107].copy_from_slice(&user_balance_base_token_amount.to_bytes());
//                 buf[3107..3115].copy_from_slice(&user_balance_quote_token_amount.to_bytes());
//                 buf[3115..3211].copy_from_slice(&user_signature.to_bytes());
//             }
//             Method::ProvideLiquidity {
//                 state_roots,
//                 earlier_proof_index,
//                 single_pool_witness,
//                 single_liquidity_witness,
//                 double_balance_witness,
//                 base_token_id,
//                 quote_token_id,
//                 user_address,
//                 user_liquidity_points,
//                 user_balance_base_token_amount,
//                 user_balance_quote_token_amount,
//                 pool_base_token_amount,
//                 pool_quote_token_amount,
//                 pool_total_liquidity_points,
//                 user_base_token_amount_to_provide,
//                 user_quote_token_amount_limit_to_provide,
//                 is_first_providing,
//                 user_signature,
//             } => {
//                 buf[0] = 4;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..797].copy_from_slice(&single_pool_witness.to_bytes());
//                 buf[797..1490].copy_from_slice(&single_liquidity_witness.to_bytes());
//                 buf[1490..2964].copy_from_slice(&double_balance_witness.to_bytes());
//                 buf[2964..2996].copy_from_slice(&base_token_id.to_bytes());
//                 buf[2996..3028].copy_from_slice(&quote_token_id.to_bytes());
//                 buf[3028..3083].copy_from_slice(&user_address.to_bytes());
//                 buf[3083..3115].copy_from_slice(&user_liquidity_points.to_bytes());
//                 buf[3115..3123].copy_from_slice(&user_balance_base_token_amount.to_bytes());
//                 buf[3123..3131].copy_from_slice(&user_balance_quote_token_amount.to_bytes());
//                 buf[3131..3139].copy_from_slice(&pool_base_token_amount.to_bytes());
//                 buf[3139..3147].copy_from_slice(&pool_quote_token_amount.to_bytes());
//                 buf[3147..3179].copy_from_slice(&pool_total_liquidity_points.to_bytes());
//                 buf[3179..3187].copy_from_slice(&user_base_token_amount_to_provide.to_bytes());
//                 buf[3187..3195]
//                     .copy_from_slice(&user_quote_token_amount_limit_to_provide.to_bytes());
//                 buf[3195] = *is_first_providing as u8;
//                 buf[3196..3292].copy_from_slice(&user_signature.to_bytes());
//             }
//             Method::RemoveLiquidity {
//                 state_roots,
//                 earlier_proof_index,
//                 single_pool_witness,
//                 single_liquidity_witness,
//                 double_balance_witness,
//                 base_token_id,
//                 quote_token_id,
//                 user_address,
//                 user_liquidity_points,
//                 user_balance_base_token_amount,
//                 user_balance_quote_token_amount,
//                 pool_base_token_amount,
//                 pool_quote_token_amount,
//                 pool_total_liquidity_points,
//                 user_liquidity_points_to_remove,
//                 user_base_token_amount_limit_to_remove,
//                 user_quote_token_amount_limit_to_remove,
//                 user_signature,
//             } => {
//                 buf[0] = 5;
//                 buf[1..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..797].copy_from_slice(&single_pool_witness.to_bytes());
//                 buf[797..1490].copy_from_slice(&single_liquidity_witness.to_bytes());
//                 buf[1490..2964].copy_from_slice(&double_balance_witness.to_bytes());
//                 buf[2964..2996].copy_from_slice(&base_token_id.to_bytes());
//                 buf[2996..3028].copy_from_slice(&quote_token_id.to_bytes());
//                 buf[3028..3083].copy_from_slice(&user_address.to_bytes());
//                 buf[3083..3115].copy_from_slice(&user_liquidity_points.to_bytes());
//                 buf[3115..3123].copy_from_slice(&user_balance_base_token_amount.to_bytes());
//                 buf[3123..3131].copy_from_slice(&user_balance_quote_token_amount.to_bytes());
//                 buf[3131..3139].copy_from_slice(&pool_base_token_amount.to_bytes());
//                 buf[3139..3147].copy_from_slice(&pool_quote_token_amount.to_bytes());
//                 buf[3147..3179].copy_from_slice(&pool_total_liquidity_points.to_bytes());
//                 buf[3179..3211].copy_from_slice(&user_liquidity_points_to_remove.to_bytes());
//                 buf[3211..3219].copy_from_slice(&user_base_token_amount_limit_to_remove.to_bytes());
//                 buf[3219..3227]
//                     .copy_from_slice(&user_quote_token_amount_limit_to_remove.to_bytes());
//                 buf[3227..3323].copy_from_slice(&user_signature.to_bytes());
//             }
//             Method::BuyTokens {
//                 state_roots,
//                 earlier_proof_index,
//                 single_pool_witness,
//                 double_balance_witness,
//                 user_address,
//                 base_token_id,
//                 quote_token_id,
//                 user_balance_base_token_amount,
//                 user_balance_quote_token_amount,
//                 pool_base_token_amount,
//                 pool_quote_token_amount,
//                 pool_total_liquidity_points,
//                 user_base_token_amount_to_swap,
//                 user_quote_token_amount_limit_to_swap,
//                 user_signature,
//             } => {
//                 buf[0] = 6;
//                 buf[888..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..797].copy_from_slice(&single_pool_witness.to_bytes());
//                 buf[797..2271].copy_from_slice(&double_balance_witness.to_bytes());
//                 buf[2271..2326].copy_from_slice(&user_address.to_bytes());
//                 buf[2326..2358].copy_from_slice(&base_token_id.to_bytes());
//                 buf[2358..2390].copy_from_slice(&quote_token_id.to_bytes());
//                 buf[2390..2398].copy_from_slice(&user_balance_base_token_amount.to_bytes());
//                 buf[2398..2406].copy_from_slice(&user_balance_quote_token_amount.to_bytes());
//                 buf[2406..2414].copy_from_slice(&pool_base_token_amount.to_bytes());
//                 buf[2414..2422].copy_from_slice(&pool_quote_token_amount.to_bytes());
//                 buf[2422..2454].copy_from_slice(&pool_total_liquidity_points.to_bytes());
//                 buf[2454..2462].copy_from_slice(&user_base_token_amount_to_swap.to_bytes());
//                 buf[2462..2470].copy_from_slice(&user_quote_token_amount_limit_to_swap.to_bytes());
//                 buf[2470..2566].copy_from_slice(&user_signature.to_bytes());
//             }
//             Method::SellTokens {
//                 state_roots,
//                 earlier_proof_index,
//                 single_pool_witness,
//                 double_balance_witness,
//                 user_address,
//                 base_token_id,
//                 quote_token_id,
//                 user_balance_base_token_amount,
//                 user_balance_quote_token_amount,
//                 pool_base_token_amount,
//                 pool_quote_token_amount,
//                 pool_total_liquidity_points,
//                 user_base_token_amount_limit_to_swap,
//                 user_quote_token_amount_to_swap,
//                 user_signature,
//             } => {
//                 buf[0] = 7;
//                 buf[888..129].copy_from_slice(&state_roots.to_bytes());
//                 buf[129..137].copy_from_slice(&earlier_proof_index.to_bytes());
//                 buf[137..797].copy_from_slice(&single_pool_witness.to_bytes());
//                 buf[797..2271].copy_from_slice(&double_balance_witness.to_bytes());
//                 buf[2271..2326].copy_from_slice(&user_address.to_bytes());
//                 buf[2326..2358].copy_from_slice(&base_token_id.to_bytes());
//                 buf[2358..2390].copy_from_slice(&quote_token_id.to_bytes());
//                 buf[2390..2398].copy_from_slice(&user_balance_base_token_amount.to_bytes());
//                 buf[2398..2406].copy_from_slice(&user_balance_quote_token_amount.to_bytes());
//                 buf[2406..2414].copy_from_slice(&pool_base_token_amount.to_bytes());
//                 buf[2414..2422].copy_from_slice(&pool_quote_token_amount.to_bytes());
//                 buf[2422..2454].copy_from_slice(&pool_total_liquidity_points.to_bytes());
//                 buf[2454..2462].copy_from_slice(&user_base_token_amount_limit_to_swap.to_bytes());
//                 buf[2462..2470].copy_from_slice(&user_quote_token_amount_to_swap.to_bytes());
//                 buf[2470..2566].copy_from_slice(&user_signature.to_bytes());
//             }
//         }

//         buf
//     }
// }
