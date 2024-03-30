import { BridgeWitnesses } from "../types/bridge-witnesses.js"
import { Sibling } from "../types/siblings.js"
import { uint256DecodeFrom } from "../utils/uint256.js"

export const parseGetBridgeWitnesses = (buffer: ArrayBuffer): BridgeWitnesses => {
    if (buffer.byteLength === 1222) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    const burnWitness: Array<Sibling> = new Array(19)
    const withdrawalWitness: Array<Sibling> = new Array(18)

    if (array[0] !== 7) {
        throw Error("Mistaken RPC response.")
    }

    for (let i = 0; i < 19; i++) {
        const padding = 1 + i * 33
        const siblingValue = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const siblingIsLeft = array[padding + 32] !== 0
        burnWitness.push({
            value: siblingValue,
            isLeft: siblingIsLeft,
        })
    }

    for (let i = 0; i < 18; i++) {
        const padding = 628 + i * 33
        const siblingValue = uint256DecodeFrom(array.subarray(padding, padding + 32))
        const siblingIsLeft = array[padding + 32] !== 0
        withdrawalWitness.push({
            value: siblingValue,
            isLeft: siblingIsLeft,
        })
    }

    return { burnWitness, withdrawalWitness }
}
