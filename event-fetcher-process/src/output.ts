import { Deposit, Withdrawal } from "nacho-common-o1js"
import { Field, PublicKey, UInt64 } from "o1js"

export const unparseOutput = (output: Array<Deposit> | Array<Withdrawal>) => {
    const arrayBuffer = new ArrayBuffer(output.length * 96)
    const buffer = new Uint8Array(arrayBuffer)

    for (let i = 0; i < output.length; i++) {
        const event = output[i]

        eventEncodeInto(event, buffer.subarray(i * 96, (i + 1) * 96))
    }

    return buffer
}

export const unparseError = () => {
    const arrayBuffer = new ArrayBuffer(1)
    const buffer = new Uint8Array(arrayBuffer)

    return buffer
}

const eventEncodeInto = (event: Deposit | Withdrawal, buffer: Uint8Array) => {
    const tag = event instanceof Deposit ? 0 : 1

    const address = event instanceof Deposit ? event.depositor : event.withdrawer

    const tokenId = event.tokenId

    const tokenAmount = event.tokenAmount

    tagEncodeInto(tag, buffer.subarray(0, 1))

    publicKeyEncodeInto(address, buffer.subarray(1, 56))

    fieldEncodeInto(tokenId, buffer.subarray(56, 88))

    uint64EncodeInto(tokenAmount, buffer.subarray(88, 96))

    return buffer
}

const tagEncodeInto = (tag: 0 | 1, buffer: Uint8Array) => {
    buffer[0] = tag
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
