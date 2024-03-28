import { Burn } from "../types/burn.js"
import { uint64DecodeFrom } from "../utils/uint64.js"

export const parseTxId = (buffer: ArrayBuffer): number => {
    if (buffer.byteLength < 9) {
        throw Error("Mistaken RPC response.")
    }

    const array = new Uint8Array(buffer)

    if (array[0] !== 8) {
        throw Error("Mistaken RPC response.")
    }

    const txId = uint64DecodeFrom(array.subarray(1, 9))

    return Number(txId)
}
