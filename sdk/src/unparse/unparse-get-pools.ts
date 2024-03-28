import { RPC_REQUEST_SIZE } from "../constants.js"

export const unparseGetPools = (): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 4

    return buffer
}
