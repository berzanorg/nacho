import { stdin, stdout } from "node:process"
import { parseInput } from "./parse"
import {
    isBurnTokensSignatureValid,
    isBuyTokensSignatureValid,
    isCreatePoolSignatureValid,
    isProvideLiquditySignatureValid,
    isRemoveLiquditySignatureValid,
    isSellTokensSignatureValid,
} from "./check"

stdin.on("data", async (chunk) => {
    const input = parseInput(chunk)

    const isValid =
        input.kind === "BurnTokens"
            ? isBurnTokensSignatureValid(input)
            : input.kind === "CreatePool"
            ? isCreatePoolSignatureValid(input)
            : input.kind === "ProvideLiquidity"
            ? isProvideLiquditySignatureValid(input)
            : input.kind === "RemoveLiquidity"
            ? isRemoveLiquditySignatureValid(input)
            : input.kind === "BuyTokens"
            ? isBuyTokensSignatureValid(input)
            : input.kind === "SellTokens"
            ? isSellTokensSignatureValid(input)
            : false

    const buffer = new ArrayBuffer(1)
    const array = new Uint8Array(buffer)
    array[0] = isValid ? 1 : 0

    stdout.write(array)
})
