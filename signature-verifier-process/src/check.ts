import { Field, PublicKey, Signature, UInt64 } from "o1js"
import {
    BurnTokens,
    BuyTokens,
    CreatePool,
    ProvideLiquidity,
    RemoveLiquidity,
    SellTokens,
} from "./input"

export const isBurnTokensSignatureValid = ({
    signature,
    address,
    token_id,
    token_amount,
}: BurnTokens) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [Field(token_id), UInt64.from(token_amount).value])
            .toBoolean()
    } catch {
        return false
    }
}

export const isCreatePoolSignatureValid = ({
    signature,
    address,
    base_token_id,
    quote_token_id,
    base_token_amount,
    quote_token_amount,
}: CreatePool) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(base_token_id),
                Field(quote_token_id),
                UInt64.from(base_token_amount).value,
                UInt64.from(quote_token_amount).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isProvideLiquditySignatureValid = ({
    signature,
    address,
    base_token_id,
    quote_token_id,
    base_token_amount,
    quote_token_amount_limit,
}: ProvideLiquidity) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(base_token_id),
                Field(quote_token_id),
                UInt64.from(base_token_amount).value,
                UInt64.from(quote_token_amount_limit).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isRemoveLiquditySignatureValid = ({
    signature,
    address,
    base_token_id,
    quote_token_id,
    base_token_amount_limit,
    quote_token_amount_limit,
    liquidity_point_amount,
}: RemoveLiquidity) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(base_token_id),
                Field(quote_token_id),
                UInt64.from(base_token_amount_limit).value,
                UInt64.from(quote_token_amount_limit).value,
                Field(liquidity_point_amount),
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isBuyTokensSignatureValid = ({
    signature,
    address,
    base_token_id,
    quote_token_id,
    base_token_amount,
    quote_token_amount_limit,
}: BuyTokens) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(base_token_id),
                Field(quote_token_id),
                UInt64.from(base_token_amount).value,
                UInt64.from(quote_token_amount_limit).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}

export const isSellTokensSignatureValid = ({
    signature,
    address,
    base_token_id,
    quote_token_id,
    base_token_amount_limit,
    quote_token_amount,
}: SellTokens) => {
    try {
        const $signature = Signature.fromFields(signature.map((field) => Field(field)))
        const $address = PublicKey.fromBase58(address)

        return $signature
            .verify($address, [
                Field(base_token_id),
                Field(quote_token_id),
                UInt64.from(base_token_amount_limit).value,
                UInt64.from(quote_token_amount).value,
            ])
            .toBoolean()
    } catch {
        return false
    }
}
