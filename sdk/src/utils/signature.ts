import { uint256EncodeInto } from "./uint256.js"

export const signatureEncodeInto = (signature: [bigint, bigint], uint8Array: Uint8Array) => {
    uint256EncodeInto(signature[0], uint8Array.subarray(0, 32))
    uint256EncodeInto(signature[0], uint8Array.subarray(32, 64))
}
