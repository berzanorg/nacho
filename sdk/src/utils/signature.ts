import { SIGNATURE_REGEXP } from "../constants.js"

export const signatureEncodeInto = (signature: string, uint8Array: Uint8Array) => {
    if (!SIGNATURE_REGEXP.test(signature)) {
        throw Error("Cannot encode invalid signature.")
    }

    new TextEncoder().encodeInto(signature, uint8Array)
}
