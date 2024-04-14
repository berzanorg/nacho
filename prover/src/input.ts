import {
    DoubleBalanceWitness,
    SingleBalanceWitness,
    SingleBurnWitness,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
} from "./types.js"

export type Input =
    | CreateGenesis
    | MergeProofs
    | DepositTokens
    | BurnTokens
    | CreatePool
    | ProvideLiquidity
    | RemoveLiquidity
    | BuyTokens
    | SellTokens
    | MistakenInput

export type CreateGenesis = {
    kind: "CreateGenesis"
    state_roots: StateRoots
}

export type MergeProofs = {
    kind: "MergeProofs"
    state_roots: StateRoots
    first_proof_index: bigint
    second_proof_index: bigint
}

export type DepositTokens = {
    kind: "DepositTokens"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_balance_witness: SingleBalanceWitness
    current_deposits_merkle_list_hash: bigint
    expected_deposits_merkle_list_hash: bigint
    user_address: string
    token_id: bigint
    user_deposit_token_amount: bigint
    user_balance_token_amount: bigint
    is_users_first_deposit: boolean
}

export type BurnTokens = {
    kind: "BurnTokens"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_balance_witness: SingleBalanceWitness
    single_burn_witness: SingleBurnWitness
    user_address: string
    token_id: bigint
    user_burn_token_amount: bigint
    user_balance_token_amount: bigint
    amount_to_burn: bigint
    user_signature: [bigint, bigint]
}

export type CreatePool = {
    kind: "CreatePool"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_pool_witness: SinglePoolWitness
    single_liquidity_witness: SingleLiquidityWitness
    double_balance_witness: DoubleBalanceWitness
    base_token_id: bigint
    quote_token_id: bigint
    user_address: string
    user_liquidity_base_token_amount: bigint
    user_liquidity_quote_token_amount: bigint
    user_balance_base_token_amount: bigint
    user_balance_quote_token_amount: bigint
    user_signature: [bigint, bigint]
}

export type ProvideLiquidity = {
    kind: "ProvideLiquidity"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_pool_witness: SinglePoolWitness
    single_liquidity_witness: SingleLiquidityWitness
    double_balance_witness: DoubleBalanceWitness
    base_token_id: bigint
    quote_token_id: bigint
    user_address: string
    user_liquidity_points: bigint
    user_balance_base_token_amount: bigint
    user_balance_quote_token_amount: bigint
    pool_base_token_amount: bigint
    pool_quote_token_amount: bigint
    pool_total_liquidity_points: bigint
    user_base_token_amount_to_provide: bigint
    user_quote_token_amount_limit_to_provide: bigint
    is_first_providing: boolean
    user_signature: [bigint, bigint]
}

export type RemoveLiquidity = {
    kind: "RemoveLiquidity"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_pool_witness: SinglePoolWitness
    single_liquidity_witness: SingleLiquidityWitness
    double_balance_witness: DoubleBalanceWitness
    base_token_id: bigint
    quote_token_id: bigint
    user_address: string
    user_liquidity_points: bigint
    user_balance_base_token_amount: bigint
    user_balance_quote_token_amount: bigint
    pool_base_token_amount: bigint
    pool_quote_token_amount: bigint
    pool_total_liquidity_points: bigint
    user_liquidity_points_to_remove: bigint
    user_base_token_amount_limit_to_remove: bigint
    user_quote_token_amount_limit_to_remove: bigint
    user_signature: [bigint, bigint]
}

export type BuyTokens = {
    kind: "BuyTokens"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_pool_witness: SinglePoolWitness
    double_balance_witness: DoubleBalanceWitness
    user_address: string
    base_token_id: bigint
    quote_token_id: bigint
    user_balance_base_token_amount: bigint
    user_balance_quote_token_amount: bigint
    pool_base_token_amount: bigint
    pool_quote_token_amount: bigint
    pool_total_liquidity_points: bigint
    user_base_token_amount_to_swap: bigint
    user_quote_token_amount_limit_to_swap: bigint
    user_signature: [bigint, bigint]
}

export type SellTokens = {
    kind: "SellTokens"
    state_roots: StateRoots
    earlier_proof_index: bigint
    single_pool_witness: SinglePoolWitness
    double_balance_witness: DoubleBalanceWitness
    user_address: string
    base_token_id: bigint
    quote_token_id: bigint
    user_balance_base_token_amount: bigint
    user_balance_quote_token_amount: bigint
    pool_base_token_amount: bigint
    pool_quote_token_amount: bigint
    pool_total_liquidity_points: bigint
    user_base_token_amount_limit_to_swap: bigint
    user_quote_token_amount_to_swap: bigint
    user_signature: [bigint, bigint]
}

export type MistakenInput = {
    kind: "MistakenInput"
}
