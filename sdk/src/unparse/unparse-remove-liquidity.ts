import { RPC_REQUEST_SIZE } from "../constants.js"
import { addressEncodeInto } from "../utils/address.js"
import { signatureEncodeInto } from "../utils/signature.js"
import { uint256EncodeInto } from "../utils/uint256.js"
import { uint64EncodeInto } from "../utils/uint64.js"

export const unparseRemoveLiquidity = (
    address: string,
    signature: string,
    baseTokenId: bigint,
    quoteTokenId: bigint,
    baseTokenAmountLimit: bigint,
    quoteTokenAmountLimit: bigint,
    liquidityPointAmount: bigint,
): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 11

    addressEncodeInto(address, array.subarray(1, 56))
    signatureEncodeInto(signature, array.subarray(56, 152))
    uint256EncodeInto(baseTokenId, array.subarray(152, 184))
    uint256EncodeInto(quoteTokenId, array.subarray(184, 216))
    uint64EncodeInto(baseTokenAmountLimit, array.subarray(216, 224))
    uint64EncodeInto(quoteTokenAmountLimit, array.subarray(224, 232))
    uint256EncodeInto(liquidityPointAmount, array.subarray(232, 264))

    return buffer
}
