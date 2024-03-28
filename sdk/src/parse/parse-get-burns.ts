import { Burn } from "../types/burn.js"
import { uint256DecodeFrom } from "../utils/uint256.js"
import { uint64DecodeFrom } from "../utils/uint64.js"

export const parseGetBurns = (buffer: ArrayBuffer): Array<Burn> => {
    if (buffer.byteLength === 0) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    const burnsLength = (buffer.byteLength - 1) / 48
    const burns: Array<Burn> = new Array(burnsLength)

    if (array[0] !== 6) {
        throw Error("Mistaken RPC response.")
    }

    for (let i = 0; i < burnsLength; i++) {
        const padding = i * 48 + 1
        const tokenId = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const amount = uint64DecodeFrom(array.subarray(padding + 32, padding + 40))
        const burnId = uint64DecodeFrom(array.subarray(padding + 40, padding + 48))

        burns.push({
            tokenId,
            amount,
            burnId,
        })
    }

    return burns
}
