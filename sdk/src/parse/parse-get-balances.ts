import { Balance } from "../types/balance.js"
import { uint256DecodeFrom } from "../utils/uint256.js"
import { uint64DecodeFrom } from "../utils/uint64.js"

export const parseGetBalances = (buffer: ArrayBuffer): Array<Balance> => {
    if (buffer.byteLength === 0) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    const balancesLength = (buffer.byteLength - 1) / 40
    const balances: Array<Balance> = new Array(balancesLength)

    if (array[0] !== 3) {
        throw Error("Mistaken RPC response.")
    }

    for (let i = 0; i < balancesLength; i++) {
        const padding = i * 40 + 1
        const tokenId = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const amount = uint64DecodeFrom(array.subarray(padding + 32, padding + 40))

        balances.push({ tokenId, amount })
    }

    return balances
}
