import {
    BurnTokens,
    BuyTokens,
    CreateGenesis,
    CreatePool,
    DepositTokens,
    Input,
    MistakenInput,
    ProvideLiquidity,
    RemoveLiquidity,
    SellTokens,
} from "./input.js"
import { Sibling, StateRoots } from "./types.js"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 3291) {
        return {
            kind: "MistakenInput",
        }
    }

    switch (array[0]) {
        case 0: {
            return {
                kind: "CreateGenesis",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
            } satisfies CreateGenesis
        }
        case 1: {
            return {
                kind: "DepositTokens",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_balance_witness: parseSingleWitness(buffer.subarray(137, 863)),
                current_deposits_merkle_list_hash: parseUint256(buffer.subarray(863, 895)),
                expected_deposits_merkle_list_hash: parseUint256(buffer.subarray(895, 927)),
                user_address: parseAddress(buffer.subarray(927, 982)),
                token_id: parseUint256(buffer.subarray(982, 1014)),
                user_deposit_token_amount: parseUint64(buffer.subarray(1014, 1022)),
                user_balance_token_amount: parseUint64(buffer.subarray(1022, 1030)),
            } satisfies DepositTokens
        }
        case 2: {
            return {
                kind: "BurnTokens",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_balance_witness: parseSingleWitness(buffer.subarray(137, 863)),
                single_burn_witness: parseSingleWitness(buffer.subarray(863, 1490)),
                user_address: parseAddress(buffer.subarray(1490, 1545)),
                token_id: parseUint256(buffer.subarray(1545, 1577)),
                user_burn_token_amount: parseUint64(buffer.subarray(1577, 1585)),
                user_balance_token_amount: parseUint64(buffer.subarray(1585, 1593)),
                amount_to_burn: parseUint64(buffer.subarray(1593, 1601)),
                user_signature: parseSignature(buffer.subarray(1601, 1665)),
            } satisfies BurnTokens
        }
        case 3: {
            return {
                kind: "CreatePool",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_pool_witness: parseSingleWitness(buffer.subarray(137, 797)),
                single_liquidity_witness: parseSingleWitness(buffer.subarray(797, 1490)),
                double_balance_witness: parseDoubleBalanceWitness(buffer.subarray(1490, 2964)),
                base_token_id: parseUint256(buffer.subarray(2964, 2996)),
                quote_token_id: parseUint256(buffer.subarray(2996, 3028)),
                user_address: parseAddress(buffer.subarray(3028, 3083)),
                user_liquidity_base_token_amount: parseUint64(buffer.subarray(3083, 3091)),
                user_liquidity_quote_token_amount: parseUint64(buffer.subarray(3091, 3099)),
                user_balance_base_token_amount: parseUint64(buffer.subarray(3099, 3107)),
                user_balance_quote_token_amount: parseUint64(buffer.subarray(3107, 3115)),
                user_signature: parseSignature(buffer.subarray(3115, 3179)),
            } satisfies CreatePool
        }
        case 4: {
            return {
                kind: "ProvideLiquidity",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_pool_witness: parseSingleWitness(buffer.subarray(137, 797)),
                single_liquidity_witness: parseSingleWitness(buffer.subarray(797, 1490)),
                double_balance_witness: parseDoubleBalanceWitness(buffer.subarray(1490, 2964)),
                base_token_id: parseUint256(buffer.subarray(2964, 2996)),
                quote_token_id: parseUint256(buffer.subarray(2996, 3028)),
                user_address: parseAddress(buffer.subarray(3028, 3083)),
                user_liquidity_points: parseUint256(buffer.subarray(3083, 3115)),
                user_balance_base_token_amount: parseUint64(buffer.subarray(3115, 3123)),
                user_balance_quote_token_amount: parseUint64(buffer.subarray(3123, 3131)),
                pool_base_token_amount: parseUint64(buffer.subarray(3131, 3139)),
                pool_quote_token_amount: parseUint64(buffer.subarray(3139, 3147)),
                pool_total_liquidity_points: parseUint256(buffer.subarray(3147, 3179)),
                user_base_token_amount_to_provide: parseUint64(buffer.subarray(3179, 3187)),
                user_quote_token_amount_limit_to_provide: parseUint64(buffer.subarray(3187, 3195)),
                user_signature: parseSignature(buffer.subarray(3195, 3259)),
            } satisfies ProvideLiquidity
        }
        case 5: {
            return {
                kind: "RemoveLiquidity",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_pool_witness: parseSingleWitness(buffer.subarray(137, 797)),
                single_liquidity_witness: parseSingleWitness(buffer.subarray(797, 1490)),
                double_balance_witness: parseDoubleBalanceWitness(buffer.subarray(1490, 2964)),
                base_token_id: parseUint256(buffer.subarray(2964, 2996)),
                quote_token_id: parseUint256(buffer.subarray(2996, 3028)),
                user_address: parseAddress(buffer.subarray(3028, 3083)),
                user_liquidity_points: parseUint256(buffer.subarray(3083, 3115)),
                user_balance_base_token_amount: parseUint64(buffer.subarray(3115, 3123)),
                user_balance_quote_token_amount: parseUint64(buffer.subarray(3123, 3131)),
                pool_base_token_amount: parseUint64(buffer.subarray(3131, 3139)),
                pool_quote_token_amount: parseUint64(buffer.subarray(3139, 3147)),
                pool_total_liquidity_points: parseUint256(buffer.subarray(3147, 3179)),
                user_liquidity_points_to_remove: parseUint256(buffer.subarray(3179, 3211)),
                user_base_token_amount_limit_to_remove: parseUint64(buffer.subarray(3211, 3219)),
                user_quote_token_amount_limit_to_remove: parseUint64(buffer.subarray(3219, 3227)),
                user_signature: parseSignature(buffer.subarray(3227, 3291)),
            } satisfies RemoveLiquidity
        }
        case 6: {
            return {
                kind: "BuyTokens",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_pool_witness: parseSingleWitness(buffer.subarray(137, 797)),
                double_balance_witness: parseDoubleBalanceWitness(buffer.subarray(797, 2271)),
                user_address: parseAddress(buffer.subarray(2271, 2326)),
                base_token_id: parseUint256(buffer.subarray(2326, 2358)),
                quote_token_id: parseUint256(buffer.subarray(2358, 2390)),
                user_balance_base_token_amount: parseUint64(buffer.subarray(2390, 2398)),
                user_balance_quote_token_amount: parseUint64(buffer.subarray(2398, 2406)),
                pool_base_token_amount: parseUint64(buffer.subarray(2406, 2414)),
                pool_quote_token_amount: parseUint64(buffer.subarray(2414, 2422)),
                pool_total_liquidity_points: parseUint256(buffer.subarray(2422, 2454)),
                user_base_token_amount_to_swap: parseUint64(buffer.subarray(2454, 2462)),
                user_quote_token_amount_limit_to_swap: parseUint64(buffer.subarray(2462, 2470)),
                user_signature: parseSignature(buffer.subarray(2470, 2534)),
            } satisfies BuyTokens
        }
        case 7: {
            return {
                kind: "SellTokens",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                earlier_proof_index: parseUint64(buffer.subarray(129, 137)),
                single_pool_witness: parseSingleWitness(buffer.subarray(137, 797)),
                double_balance_witness: parseDoubleBalanceWitness(buffer.subarray(797, 2271)),
                user_address: parseAddress(buffer.subarray(2271, 2326)),
                base_token_id: parseUint256(buffer.subarray(2326, 2358)),
                quote_token_id: parseUint256(buffer.subarray(2358, 2390)),
                user_balance_base_token_amount: parseUint64(buffer.subarray(2390, 2398)),
                user_balance_quote_token_amount: parseUint64(buffer.subarray(2398, 2406)),
                pool_base_token_amount: parseUint64(buffer.subarray(2406, 2414)),
                pool_quote_token_amount: parseUint64(buffer.subarray(2414, 2422)),
                pool_total_liquidity_points: parseUint256(buffer.subarray(2422, 2454)),
                user_base_token_amount_limit_to_swap: parseUint64(buffer.subarray(2454, 2462)),
                user_quote_token_amount_to_swap: parseUint64(buffer.subarray(2462, 2470)),
                user_signature: parseSignature(buffer.subarray(2470, 2534)),
            } satisfies SellTokens
        }
        default: {
            return {
                kind: "MistakenInput",
            } satisfies MistakenInput
        }
    }
}

const parseAddress = (array: Uint8Array) => {
    return new TextDecoder().decode(array)
}

const parseSignature = (array: Uint8Array) => {
    const r = parseUint256(array.subarray(0, 32))
    const s = parseUint256(array.subarray(32, 64))

    const signature = [r, s] satisfies [bigint, bigint]

    return signature
}

const parseBoolean = (array: Uint8Array) => {
    return array[0] !== 0
}

const parseUint64 = (array: Uint8Array) => {
    let result = 0n
    for (let i = 0; i < 8; i++) {
        result |= BigInt(array[i]) << BigInt(i * 8)
    }

    return result
}

const parseUint256 = (array: Uint8Array) => {
    let result = 0n
    for (let i = 0; i < 32; i++) {
        result |= BigInt(array[i]) << BigInt(i * 8)
    }

    return result
}

const parseStateRoots = (array: Uint8Array) => {
    const stateRoots = {
        balances: parseUint256(array.subarray(0, 32)),
        liquidities: parseUint256(array.subarray(32, 64)),
        pools: parseUint256(array.subarray(64, 96)),
        burns: parseUint256(array.subarray(96, 128)),
    } satisfies StateRoots

    return stateRoots
}

const parseSibling = (array: Uint8Array) => {
    const sibling = {
        value: parseUint256(array.subarray(0, 32)),
        isLeft: parseBoolean(array.subarray(32, 33)),
    } satisfies Sibling

    return sibling
}

const parseSingleWitness = (array: Uint8Array) => {
    const siblings: Array<Sibling> = []

    for (let i = 0; i < array.length; i += 33) {
        siblings.push(parseSibling(array.subarray(i, i + 33)))
    }

    const singleWitness = { siblings }

    return singleWitness
}

const parseDoubleBalanceWitness = (array: Uint8Array) => {
    const siblingsX1: Array<Sibling> = []
    const siblingsX2: Array<Sibling> = []
    const siblingsAt: Array<boolean> = []

    for (let i = 0; i < 726; i += 33) {
        siblingsX1.push(parseSibling(array.subarray(i, i + 33)))
    }

    for (let i = 726; i < 1452; i += 33) {
        siblingsX1.push(parseSibling(array.subarray(i, i + 33)))
    }

    for (let i = 1452; i < 1474; i += 1) {
        siblingsAt.push(parseBoolean(array.subarray(i, i + 1)))
    }

    const doubleWitness = {
        siblingsX1,
        siblingsX2,
        siblingsAt,
    }

    return doubleWitness
}
