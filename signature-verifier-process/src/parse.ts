import {
    BurnTokens,
    BuyTokens,
    CreatePool,
    Input,
    MistakenInput,
    ProvideLiquidity,
    RemoveLiquidity,
    SellTokens,
} from "./input"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 232) {
        return {
            kind: "MistakenInput",
        } satisfies MistakenInput
    }

    switch (array[0]) {
        case 0: {
            return {
                kind: "BurnTokens",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                token_id: parseUint256(buffer.subarray(120, 152)),
                token_amount: parseUint64(buffer.subarray(152, 160)),
            } satisfies BurnTokens
        }
        case 1: {
            return {
                kind: "CreatePool",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                base_token_id: parseUint256(buffer.subarray(120, 152)),
                quote_token_id: parseUint256(buffer.subarray(152, 184)),
                base_token_amount: parseUint64(buffer.subarray(184, 192)),
                quote_token_amount: parseUint64(buffer.subarray(192, 200)),
            } satisfies CreatePool
        }
        case 2: {
            return {
                kind: "ProvideLiquidity",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                base_token_id: parseUint256(buffer.subarray(120, 152)),
                quote_token_id: parseUint256(buffer.subarray(152, 184)),
                base_token_amount: parseUint64(buffer.subarray(184, 192)),
                quote_token_amount_limit: parseUint64(buffer.subarray(192, 200)),
            } satisfies ProvideLiquidity
        }
        case 3: {
            return {
                kind: "RemoveLiquidity",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                base_token_id: parseUint256(buffer.subarray(120, 152)),
                quote_token_id: parseUint256(buffer.subarray(152, 184)),
                base_token_amount_limit: parseUint64(buffer.subarray(184, 192)),
                quote_token_amount_limit: parseUint64(buffer.subarray(192, 200)),
                liquidity_point_amount: parseUint256(buffer.subarray(200, 232)),
            } satisfies RemoveLiquidity
        }
        case 4: {
            return {
                kind: "BuyTokens",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                base_token_id: parseUint256(buffer.subarray(120, 152)),
                quote_token_id: parseUint256(buffer.subarray(152, 184)),
                base_token_amount: parseUint64(buffer.subarray(184, 192)),
                quote_token_amount_limit: parseUint64(buffer.subarray(192, 200)),
            } satisfies BuyTokens
        }
        case 5: {
            return {
                kind: "SellTokens",
                address: parseAddress(buffer.subarray(1, 56)),
                signature: parseSignature(buffer.subarray(56, 120)),
                base_token_id: parseUint256(buffer.subarray(120, 152)),
                quote_token_id: parseUint256(buffer.subarray(152, 184)),
                base_token_amount_limit: parseUint64(buffer.subarray(184, 192)),
                quote_token_amount: parseUint64(buffer.subarray(192, 200)),
            } satisfies SellTokens
        }
        default: {
            return {
                kind: "MistakenInput",
            } satisfies MistakenInput
        }
    }
}

const parseSignature = (array: Uint8Array) => {
    const r = parseUint256(array.subarray(0, 32))
    const s = parseUint256(array.subarray(32, 64))

    const signature = [r, s] satisfies [bigint, bigint]

    return signature
}

const parseAddress = (array: Uint8Array) => {
    return new TextDecoder().decode(array)
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
