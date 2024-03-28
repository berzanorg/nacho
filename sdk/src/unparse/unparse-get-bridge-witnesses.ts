import { RPC_REQUEST_SIZE } from "../constants.js"

export const unparseGetBridgeWitnesses = (burnId: number): ArrayBuffer => {
    const buffer = new ArrayBuffer(RPC_REQUEST_SIZE)
    const view = new DataView(buffer)

    view.setUint8(0, 7)
    view.setBigUint64(1, BigInt(burnId))

    return buffer
}
