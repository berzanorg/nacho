import { Deposit, Withdrawal } from "nacho-common-o1js"
import { Field, PublicKey, UInt64 } from "o1js"

export const unparseOutput = (
    events: Array<Deposit> | Array<Withdrawal>,
    last_fetched_block: number,
) => {
    const arrayBuffer = new ArrayBuffer(8 + events.length * 95)
    const buffer = new Uint8Array(arrayBuffer)

    uint32EncodeInto(last_fetched_block, buffer.subarray(0, 4))
    uint32EncodeInto(events.length, buffer.subarray(4, 8))

    for (let i = 0; i < events.length; i++) {
        const event = events[i]

        eventEncodeInto(event, buffer.subarray(8 + i * 95, 8 + (i + 1) * 95))
    }

    return buffer
}

export const unparseError = () => {
    const arrayBuffer = new ArrayBuffer(1)
    const buffer = new Uint8Array(arrayBuffer)

    return buffer
}

const eventEncodeInto = (event: Deposit | Withdrawal, buffer: Uint8Array) => {
    const address = event instanceof Deposit ? event.depositor : event.withdrawer

    const tokenId = event.tokenId

    const tokenAmount = event.tokenAmount

    publicKeyEncodeInto(address, buffer.subarray(0, 55))

    fieldEncodeInto(tokenId, buffer.subarray(55, 87))

    uint64EncodeInto(tokenAmount, buffer.subarray(87, 95))

    return buffer
}

const publicKeyEncodeInto = (publicKey: PublicKey, buffer: Uint8Array) => {
    new TextEncoder().encodeInto(publicKey.toBase58(), buffer)
}

const fieldEncodeInto = (field: Field, uint8Array: Uint8Array) => {
    let number = field.toBigInt()

    for (let i = 0; i < 32; i++) {
        uint8Array[i] = Number(number & 0xffn)
        number >>= 8n
    }
}

const uint64EncodeInto = (uint64: UInt64, uint8Array: Uint8Array) => {
    let number = uint64.toBigInt()

    for (let i = 0; i < 8; i++) {
        uint8Array[i] = Number(number & 0xffn)
        number >>= 8n
    }
}

const uint32EncodeInto = (uint32: number, uint8Array: Uint8Array) => {
    let number = BigInt(uint32)

    for (let i = 0; i < 4; i++) {
        uint8Array[i] = Number(number & 0xffn)
        number >>= 8n
    }
}
