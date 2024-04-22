import { Input, MergeProofs, MistakenInput } from "./input.js"
import { StateRoots } from "./types.js"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 145) {
        return {
            kind: "MistakenInput",
        }
    }

    switch (array[0]) {
        case 0: {
            return {
                kind: "MergeProofs",
                state_roots: parseStateRoots(buffer.subarray(1, 129)),
                first_proof_index: parseUint64(buffer.subarray(129, 137)),
                second_proof_index: parseUint64(buffer.subarray(137, 145)),
            } satisfies MergeProofs
        }
        default: {
            return {
                kind: "MistakenInput",
            } satisfies MistakenInput
        }
    }
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

const parseStateRoots = (array: Uint8Array) => {
    const stateRoots = {
        balances: parseUint256(array.subarray(0, 32)),
        liquidities: parseUint256(array.subarray(32, 64)),
        pools: parseUint256(array.subarray(64, 96)),
        burns: parseUint256(array.subarray(96, 128)),
    } satisfies StateRoots

    return stateRoots
}
