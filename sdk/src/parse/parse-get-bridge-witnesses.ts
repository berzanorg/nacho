import { BridgeWitnesses } from "../types/bridge-witnesses.js"

export const parseGetBridgeWitnesses = (buffer: ArrayBuffer): BridgeWitnesses => {
    if (buffer.byteLength === 0) {
        throw Error("Mistaken response data.")
    }

    const array = new Uint8Array(buffer)

    // TODO: parse bridge witnesses

    return null as never
}
