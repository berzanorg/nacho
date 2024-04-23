import { RPC_REQUEST_SIZE } from "../constants.js"
import { addressEncodeInto } from "../utils/address.js"
import { signatureEncodeInto } from "../utils/signature.js"
import { uint256EncodeInto } from "../utils/uint256.js"
import { uint64EncodeInto } from "../utils/uint64.js"

export const unparseBuyTokens = (
    address: string,
    signature: [bigint, bigint],
    baseTokenId: bigint,
    quoteTokenId: bigint,
    baseTokenAmount: bigint,
    quoteTokenAmountLimit: bigint,
): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 12

    addressEncodeInto(address, array.subarray(1, 56))
    signatureEncodeInto(signature, array.subarray(56, 120))
    uint256EncodeInto(baseTokenId, array.subarray(120, 152))
    uint256EncodeInto(quoteTokenId, array.subarray(152, 184))
    uint64EncodeInto(baseTokenAmount, array.subarray(184, 192))
    uint64EncodeInto(quoteTokenAmountLimit, array.subarray(192, 200))

    return buffer
}
