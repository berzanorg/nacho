import { Liquidity } from "../types/liquidity.js"
import { uint256DecodeFrom } from "../utils/uint256.js"

export const parseGetLiquidities = (buffer: ArrayBuffer): Array<Liquidity> => {
    if (buffer.byteLength === 0) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    const liquiditiesLength = (buffer.byteLength - 1) / 96
    const liquidities: Array<Liquidity> = new Array(liquiditiesLength)

    if (array[0] !== 5) {
        throw Error("Mistaken RPC response.")
    }

    for (let i = 0; i < liquiditiesLength; i++) {
        const padding = i * 96 + 1
        const baseTokenId = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const quoteTokenId = uint256DecodeFrom(array.subarray(padding + 32, padding + 64))
        const points = uint256DecodeFrom(array.subarray(padding + 64, padding + 96))

        liquidities.push({
            baseTokenId,
            quoteTokenId,
            points,
        })
    }

    return liquidities
}
