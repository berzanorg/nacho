import { ADDRESS_REGEXP } from "../constants.js"

export const addressEncodeInto = (address: string, uint8Array: Uint8Array) => {
    if (!ADDRESS_REGEXP.test(address)) {
        throw Error("Cannot encode invalid address.")
    }

    new TextEncoder().encodeInto(address, uint8Array)
}
