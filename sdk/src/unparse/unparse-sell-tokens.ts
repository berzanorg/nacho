import { RPC_REQUEST_SIZE } from "../constants.js"
import { addressEncodeInto } from "../utils/address.js"
import { signatureEncodeInto } from "../utils/signature.js"
import { uint256EncodeInto } from "../utils/uint256.js"
import { uint64EncodeInto } from "../utils/uint64.js"

export const unparseSellTokens = (
    address: string,
    signature: string,
    baseTokenId: bigint,
    quoteTokenId: bigint,
    baseTokenAmountLimit: bigint,
    quoteTokenAmount: bigint,
): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 13

    addressEncodeInto(address, array.subarray(1, 56))
    signatureEncodeInto(signature, array.subarray(56, 152))
    uint256EncodeInto(baseTokenId, array.subarray(152, 184))
    uint256EncodeInto(quoteTokenId, array.subarray(184, 216))
    uint64EncodeInto(baseTokenAmountLimit, array.subarray(216, 224))
    uint64EncodeInto(quoteTokenAmount, array.subarray(224, 232))

    return buffer
}
