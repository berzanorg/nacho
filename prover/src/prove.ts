import {
    DoubleBalanceWitness,
    SingleBalanceWitness,
    SingleBurnWitness,
    SingleLiquidityWitness,
    SinglePoolWitness,
    StateRoots,
} from "nacho-common-o1js"
import { proofGenerator } from "nacho-proof-generator"
import { Bool, Field, PublicKey, Scalar, Signature, UInt64 } from "o1js"
import {
    BurnTokens,
    BuyTokens,
    CreateGenesis,
    CreatePool,
    DepositTokens,
    MergeProofs,
    ProvideLiquidity,
    RemoveLiquidity,
    SellTokens,
} from "./input.js"
import { readProofFromDisk } from "./utils.js"

export const createGenesis = async (params: CreateGenesis) => {
    const proof = await proofGenerator.createGenesis(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
    )

    return proof
}

export const mergeProofs = async (params: MergeProofs, proofDbPath: string) => {
    const proof = await proofGenerator.mergeProofs(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.first_proof_index),
        await readProofFromDisk(proofDbPath, params.second_proof_index),
    )

    return proof
}

export const depositTokens = async (params: DepositTokens, proofDbPath: string) => {
    const proof = await proofGenerator.depositTokens(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SingleBalanceWitness(params.single_balance_witness.siblings),
        Field(params.current_deposits_merkle_list_hash),
        Field(params.expected_deposits_merkle_list_hash),
        PublicKey.fromBase58(params.user_address),
        Field(params.token_id),
        UInt64.from(params.user_deposit_token_amount),
        UInt64.from(params.user_balance_token_amount),
        Bool(params.is_users_first_deposit),
    )

    return proof
}

export const makeBurnTokens = async (params: BurnTokens, proofDbPath: string) => {
    const proof = await proofGenerator.makeBurnTokens(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SingleBalanceWitness(params.single_balance_witness.siblings),
        new SingleBurnWitness(params.single_burn_witness.siblings),
        PublicKey.fromBase58(params.user_address),
        Field(params.token_id),
        UInt64.from(params.user_burn_token_amount),
        UInt64.from(params.user_balance_token_amount),
        UInt64.from(params.amount_to_burn),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}

export const makeCreatePool = async (params: CreatePool, proofDbPath: string) => {
    const proof = await proofGenerator.makeCreatePool(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SinglePoolWitness(params.single_pool_witness.siblings),
        new SingleLiquidityWitness(params.single_liquidity_witness.siblings),
        new DoubleBalanceWitness(
            params.double_balance_witness.siblingsX1,
            params.double_balance_witness.siblingsX2,
            params.double_balance_witness.siblingsAt,
        ),
        Field(params.base_token_id),
        Field(params.quote_token_id),
        PublicKey.fromBase58(params.user_address),
        UInt64.from(params.user_liquidity_base_token_amount),
        UInt64.from(params.user_liquidity_quote_token_amount),
        UInt64.from(params.user_balance_base_token_amount),
        UInt64.from(params.user_balance_quote_token_amount),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}

export const makeProvideLiquidity = async (params: ProvideLiquidity, proofDbPath: string) => {
    const proof = await proofGenerator.makeProvideLiquidity(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SinglePoolWitness(params.single_pool_witness.siblings),
        new SingleLiquidityWitness(params.single_liquidity_witness.siblings),
        new DoubleBalanceWitness(
            params.double_balance_witness.siblingsX1,
            params.double_balance_witness.siblingsX2,
            params.double_balance_witness.siblingsAt,
        ),
        Field(params.base_token_id),
        Field(params.quote_token_id),
        PublicKey.fromBase58(params.user_address),
        Field(params.user_liquidity_points),
        UInt64.from(params.user_balance_base_token_amount),
        UInt64.from(params.user_balance_quote_token_amount),
        UInt64.from(params.pool_base_token_amount),
        UInt64.from(params.pool_quote_token_amount),
        Field(params.pool_total_liquidity_points),
        UInt64.from(params.user_base_token_amount_to_provide),
        UInt64.from(params.user_quote_token_amount_limit_to_provide),
        Bool(params.is_first_providing),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}

export const makeRemoveLiquidity = async (params: RemoveLiquidity, proofDbPath: string) => {
    const proof = await proofGenerator.makeRemoveLiquidity(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SinglePoolWitness(params.single_pool_witness.siblings),
        new SingleLiquidityWitness(params.single_liquidity_witness.siblings),
        new DoubleBalanceWitness(
            params.double_balance_witness.siblingsX1,
            params.double_balance_witness.siblingsX2,
            params.double_balance_witness.siblingsAt,
        ),
        Field(params.base_token_id),
        Field(params.quote_token_id),
        PublicKey.fromBase58(params.user_address),
        Field(params.user_liquidity_points),
        UInt64.from(params.user_balance_base_token_amount),
        UInt64.from(params.user_balance_quote_token_amount),
        UInt64.from(params.pool_base_token_amount),
        UInt64.from(params.pool_quote_token_amount),
        Field(params.pool_total_liquidity_points),
        Field(params.user_liquidity_points_to_remove),
        UInt64.from(params.user_base_token_amount_limit_to_remove),
        UInt64.from(params.user_quote_token_amount_limit_to_remove),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}

export const makeBuyTokens = async (params: BuyTokens, proofDbPath: string) => {
    const proof = await proofGenerator.makeBuyTokens(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SinglePoolWitness(params.single_pool_witness.siblings),
        new DoubleBalanceWitness(
            params.double_balance_witness.siblingsX1,
            params.double_balance_witness.siblingsX2,
            params.double_balance_witness.siblingsAt,
        ),
        PublicKey.fromBase58(params.user_address),
        Field(params.base_token_id),
        Field(params.quote_token_id),
        UInt64.from(params.user_balance_base_token_amount),
        UInt64.from(params.user_balance_quote_token_amount),
        UInt64.from(params.pool_base_token_amount),
        UInt64.from(params.pool_quote_token_amount),
        Field(params.pool_total_liquidity_points),
        UInt64.from(params.user_base_token_amount_to_swap),
        UInt64.from(params.user_quote_token_amount_limit_to_swap),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}

export const makeSellTokens = async (params: SellTokens, proofDbPath: string) => {
    const proof = await proofGenerator.makeSellTokens(
        new StateRoots({
            balances: Field(params.state_roots.balances),
            liquidities: Field(params.state_roots.liquidities),
            pools: Field(params.state_roots.pools),
            burns: Field(params.state_roots.burns),
        }),
        await readProofFromDisk(proofDbPath, params.earlier_proof_index),
        new SinglePoolWitness(params.single_pool_witness.siblings),
        new DoubleBalanceWitness(
            params.double_balance_witness.siblingsX1,
            params.double_balance_witness.siblingsX2,
            params.double_balance_witness.siblingsAt,
        ),
        PublicKey.fromBase58(params.user_address),
        Field(params.base_token_id),
        Field(params.quote_token_id),
        UInt64.from(params.user_balance_base_token_amount),
        UInt64.from(params.user_balance_quote_token_amount),
        UInt64.from(params.pool_base_token_amount),
        UInt64.from(params.pool_quote_token_amount),
        Field(params.pool_total_liquidity_points),
        UInt64.from(params.user_base_token_amount_limit_to_swap),
        UInt64.from(params.user_quote_token_amount_to_swap),
        Signature.fromObject({
            r: Field.from(params.user_signature[0]),
            s: Scalar.from(params.user_signature[1]),
        }),
    )

    return proof
}
