import { Field, PublicKey, Signature, UInt64 } from "o1js"

export const isBurnTokensSignatureValid = ({
    signature,
    address,
    tokenId,
    tokenAmount,
}: {
    signature: string
    address: string
    tokenId: bigint
    tokenAmount: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [Field(tokenId), UInt64.from(tokenAmount).value])
            .toBoolean()
    } catch {
        return false
    }
}

export const isCreatePoolSignatureValid = ({
    signature,
    address,
    baseTokenId,
    quoteTokenId,
    baseTokenAmount,
    quoteTokenAmount,
}: {
    signature: string
    address: string
    baseTokenId: bigint
    quoteTokenId: bigint
    baseTokenAmount: bigint
    quoteTokenAmount: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(baseTokenId),
                Field(quoteTokenId),
                UInt64.from(baseTokenAmount).value,
                UInt64.from(quoteTokenAmount).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isProvideLiquditySignatureValid = ({
    signature,
    address,
    baseTokenId,
    quoteTokenId,
    baseTokenAmount,
    quoteTokenAmountLimit,
}: {
    signature: string
    address: string
    baseTokenId: bigint
    quoteTokenId: bigint
    baseTokenAmount: bigint
    quoteTokenAmountLimit: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(baseTokenId),
                Field(quoteTokenId),
                UInt64.from(baseTokenAmount).value,
                UInt64.from(quoteTokenAmountLimit).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isRemoveLiquditySignatureValid = ({
    signature,
    address,
    baseTokenId,
    quoteTokenId,
    baseTokenAmountLimit,
    quoteTokenAmountLimit,
    liquidityPoints,
}: {
    signature: string
    address: string
    baseTokenId: bigint
    quoteTokenId: bigint
    baseTokenAmountLimit: bigint
    quoteTokenAmountLimit: bigint
    liquidityPoints: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(baseTokenId),
                Field(quoteTokenId),
                UInt64.from(baseTokenAmountLimit).value,
                UInt64.from(quoteTokenAmountLimit).value,
                Field(liquidityPoints),
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isBuyTokensSignatureValid = ({
    signature,
    address,
    baseTokenId,
    quoteTokenId,
    baseTokenAmount,
    quoteTokenAmountLimit,
}: {
    signature: string
    address: string
    baseTokenId: bigint
    quoteTokenId: bigint
    baseTokenAmount: bigint
    quoteTokenAmountLimit: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(baseTokenId),
                Field(quoteTokenId),
                UInt64.from(baseTokenAmount).value,
                UInt64.from(quoteTokenAmountLimit).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isSellTokensSignatureValid = ({
    signature,
    address,
    baseTokenId,
    quoteTokenId,
    baseTokenAmountLimit,
    quoteTokenAmount,
}: {
    signature: string
    address: string
    baseTokenId: bigint
    quoteTokenId: bigint
    baseTokenAmountLimit: bigint
    quoteTokenAmount: bigint
}) => {
    try {
        const $signature = Signature.fromBase58(signature)
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(baseTokenId),
                Field(quoteTokenId),
                UInt64.from(baseTokenAmountLimit).value,
                UInt64.from(quoteTokenAmount).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}
