import { RPC_REQUEST_SIZE } from "../constants.js"
import { addressEncodeInto } from "../utils/address.js"
import { signatureEncodeInto } from "../utils/signature.js"
import { uint256EncodeInto } from "../utils/uint256.js"
import { uint64EncodeInto } from "../utils/uint64.js"

export const unparseBurnTokens = (
    address: string,
    signature: [bigint, bigint],
    tokenId: bigint,
    amount: bigint,
): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const array = new Uint8Array(buffer)

    array[0] = 8

    addressEncodeInto(address, array.subarray(1, 56))
    signatureEncodeInto(signature, array.subarray(56, 120))
    uint256EncodeInto(tokenId, array.subarray(120, 152))
    uint64EncodeInto(amount, array.subarray(152, 160))

    return buffer
}
