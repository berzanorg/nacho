import { Input } from "./input.js"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 264) {
        return {
            kind: "mistaken",
        }
    }

    switch (array[0]) {
        case 0: {
            return {
                kind: "burnTokens",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                tokenId: parseUint256(buffer.subarray(152, 184)),
                tokenAmount: parseUint64(buffer.subarray(184, 192)),
            }
        }
        case 1: {
            return {
                kind: "createPool",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                baseTokenId: parseUint256(buffer.subarray(152, 184)),
                quoteTokenId: parseUint256(buffer.subarray(184, 216)),
                baseTokenAmount: parseUint64(buffer.subarray(216, 224)),
                quoteTokenAmount: parseUint64(buffer.subarray(224, 232)),
            }
        }
        case 2: {
            return {
                kind: "provideLiquidity",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                baseTokenId: parseUint256(buffer.subarray(152, 184)),
                quoteTokenId: parseUint256(buffer.subarray(184, 216)),
                baseTokenAmount: parseUint64(buffer.subarray(216, 224)),
                quoteTokenAmountLimit: parseUint64(buffer.subarray(224, 232)),
            }
        }
        case 3: {
            return {
                kind: "removeLiquidity",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                baseTokenId: parseUint256(buffer.subarray(152, 184)),
                quoteTokenId: parseUint256(buffer.subarray(184, 216)),
                baseTokenAmountLimit: parseUint64(buffer.subarray(216, 224)),
                quoteTokenAmountLimit: parseUint64(buffer.subarray(224, 232)),
                liquidityPoints: parseUint256(buffer.subarray(232, 264)),
            }
        }
        case 4: {
            return {
                kind: "buyTokens",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                baseTokenId: parseUint256(buffer.subarray(152, 184)),
                quoteTokenId: parseUint256(buffer.subarray(184, 216)),
                baseTokenAmount: parseUint64(buffer.subarray(216, 224)),
                quoteTokenAmountLimit: parseUint64(buffer.subarray(224, 232)),
            }
        }
        case 5: {
            return {
                kind: "sellTokens",
                signature: parseSignature(buffer.subarray(1, 97)),
                address: parseAddress(buffer.subarray(97, 152)),
                baseTokenId: parseUint256(buffer.subarray(152, 184)),
                quoteTokenId: parseUint256(buffer.subarray(184, 216)),
                baseTokenAmountLimit: parseUint64(buffer.subarray(216, 224)),
                quoteTokenAmount: parseUint64(buffer.subarray(224, 232)),
            }
        }
        default: {
            return {
                kind: "mistaken",
            }
        }
    }
}

const parseSignature = (array: Uint8Array) => {
    return new TextDecoder().decode(array)
}

const parseAddress = (array: Uint8Array) => {
    return new TextDecoder().decode(array)
}

const parseUint64 = (array: Uint8Array) => {
    let result = 0n
    for (let i = 0; i < 8; i++) {
        result |= BigInt(array[i]) << BigInt(i * 8)
    }

    return result
}

const parseUint256 = (array: Uint8Array) => {
    let result = 0n
    for (let i = 0; i < 32; i++) {
        result |= BigInt(array[i]) << BigInt(i * 8)
    }

    return result
}
