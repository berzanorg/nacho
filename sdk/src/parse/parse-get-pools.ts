import { Pool } from "../types/pool.js"
import { uint256DecodeFrom } from "../utils/uint256.js"
import { uint64DecodeFrom } from "../utils/uint64.js"

export const parseGetPools = (buffer: ArrayBuffer): Array<Pool> => {
    if (buffer.byteLength === 0) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    const poolsLength = (buffer.byteLength - 1) / 112
    const pools: Array<Pool> = new Array(poolsLength)

    if (array[0] !== 4) {
        throw Error("Mistaken RPC response.")
    }

    for (let i = 0; i < poolsLength; i++) {
        const padding = i * 112 + 1
        const baseTokenId = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const quoteTokenId = uint256DecodeFrom(array.subarray(padding + 32, padding + 64))
        const baseTokenAmount = uint64DecodeFrom(array.subarray(padding + 64, padding + 72))
        const quoteTokenAmount = uint64DecodeFrom(array.subarray(padding + 72, padding + 80))
        const totalLiquidityPoints = uint256DecodeFrom(array.subarray(padding + 80, padding + 112))

        pools.push({
            baseTokenId,
            quoteTokenId,
            baseTokenAmount,
            quoteTokenAmount,
            totalLiquidityPoints,
        })
    }

    return pools
}
