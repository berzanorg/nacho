import { Input } from "./input"

export const parseInput = (buffer: Buffer): Input => {
    const array = new Uint8Array(buffer)

    if (array.length !== 5) {
        return {
            kind: "MistakenInput",
        }
    }

    switch (array[0]) {
        case 0: {
            return {
                kind: "FetchDepositedEvents",
                fromBlock: parseUint32(buffer.subarray(1, 5)),
            }
        }
        case 1: {
            return {
                kind: "FetchWithdrawnEvents",
                fromBlock: parseUint32(buffer.subarray(1, 5)),
            }
        }
        default: {
            return {
                kind: "MistakenInput",
            }
        }
    }
}

const parseUint32 = (array: Uint8Array) => {
    let result = 0
    for (let i = 0; i < 4; i++) {
        result |= array[i] << (i * 8)
    }

    return result
}
