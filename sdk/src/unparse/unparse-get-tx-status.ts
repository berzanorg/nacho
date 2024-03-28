import { RPC_REQUEST_SIZE } from "../constants.js"
import { uint64EncodeInto } from "../utils/uint64.js"

export const unparseGetTxStatus = (txId: number): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 2

    uint64EncodeInto(BigInt(txId), array.subarray(1, 9))

    return buffer
}
