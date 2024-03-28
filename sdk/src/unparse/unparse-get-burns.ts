import { RPC_REQUEST_SIZE } from "../constants.js"
import { addressEncodeInto } from "../utils/address.js"

export const unparseGetBurns = (address: string): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 6

    addressEncodeInto(address, array.subarray(1, 56))

    return buffer
}
