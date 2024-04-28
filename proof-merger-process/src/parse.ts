import { Input } from "./input"
import { StateRoots } from "./types"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 132) {
        return {
            kind: "MistakenInput",
        }
    }

    return {
        kind: "MergeWithPrevious",
        state_roots: parseStateRoots(array.subarray(0, 128)),
        proof_index: parseUint32(array.subarray(128, 132)),
    }
}

const parseUint32 = (array: Uint8Array) => {
    let result = 0
    for (let i = 0; i < 4; i++) {
        result |= array[i] << (i * 8)
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

const parseStateRoots = (array: Uint8Array) => {
    const stateRoots = {
        balances: parseUint256(array.subarray(0, 32)),
        liquidities: parseUint256(array.subarray(32, 64)),
        pools: parseUint256(array.subarray(64, 96)),
        burns: parseUint256(array.subarray(96, 128)),
    } satisfies StateRoots

    return stateRoots
}
