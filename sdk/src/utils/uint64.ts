export const uint64EncodeInto = (number: bigint, uint8Array: Uint8Array) => {
    if (number < 0n || number >= 2n ** 64n) {
        throw Error("Cannot encode invalid uint64.")
    }

    for (let i = 0; i < 8; i++) {
        uint8Array[i] = Number(number & 0xffn)
        number >>= 8n
    }
}

export const uint64DecodeFrom = (uint8Array: Uint8Array) => {
    if (uint8Array.length !== 8) {
        throw Error("Cannot decode invalid uint64.")
    }

    let result = 0n
    for (let i = 0; i < 8; i++) {
        result |= BigInt(uint8Array[i]) << BigInt(i * 8)
    }

    return result
}
