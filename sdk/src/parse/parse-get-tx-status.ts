import { TxStatus } from "../types/tx-status.js"

export const parseTxStatus = (buffer: ArrayBuffer): TxStatus => {
    if (buffer.byteLength < 2) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    if (array[0] !== 2) {
        throw Error("Mistaken RPC response.")
    }

    switch (array[1]) {
        case 0: {
            return TxStatus.Rejected
        }
        case 1: {
            return TxStatus.Executed
        }
        case 2: {
            return TxStatus.Proved
        }
        case 3: {
            return TxStatus.Settled
        }
        default: {
            throw Error("Mistaken transaction status.")
        }
    }
}
