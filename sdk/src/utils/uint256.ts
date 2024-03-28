export const uint256EncodeInto = (number: bigint, uint8Array: Uint8Array) => {
    if (number < 0n || number >= 2n ** 256n) {
        throw Error("Cannot encode invalid uint256.")
    }

    for (let i = 0; i < 32; i++) {
        uint8Array[i] = Number(number & 0xffn)
        number >>= 8n
    }
}

export const uint256DecodeFrom = (uint8Array: Uint8Array) => {
    if (uint8Array.length !== 32) {
        throw Error("Cannot decode invalid uint256.")
    }

    let result = 0n
    for (let i = 0; i < 32; i++) {
        result |= BigInt(uint8Array[i]) << BigInt(i * 8)
    }

    return result
}
