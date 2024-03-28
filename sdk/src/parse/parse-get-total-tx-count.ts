import { uint64DecodeFrom } from "../utils/uint64.js"

export const parseGetTotalTxCount = (buffer: ArrayBuffer): number => {
    if (buffer.byteLength < 8) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    if (array[0] !== 1) {
        throw Error("Mistaken RPC response.")
    }

    const totalTxCount = uint64DecodeFrom(array.subarray(1, 9))

    return Number(totalTxCount)
}
