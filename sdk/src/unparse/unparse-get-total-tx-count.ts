import { RPC_REQUEST_SIZE } from "../constants.js"

export const unparseGetTotalTxCount = (): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 1

    return buffer
}
