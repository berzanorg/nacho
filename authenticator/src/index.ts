import { stdin, stdout } from "node:process"
import { parseInput } from "./parse.js"
import {
    isBurnTokensSignatureValid,
    isBuyTokensSignatureValid,
    isCreatePoolSignatureValid,
    isProvideLiquditySignatureValid,
    isRemoveLiquditySignatureValid,
    isSellTokensSignatureValid,
} from "./check.js"

const main = async () => {
    stdin.on("data", async (chunk) => {
        const input = parseInput(chunk)

        const isValid =
            input.kind === "mistaken"
                ? false
                : input.kind === "burnTokens"
                ? isBurnTokensSignatureValid(input)
                : input.kind === "createPool"
                ? isCreatePoolSignatureValid(input)
                : input.kind === "provideLiquidity"
                ? isProvideLiquditySignatureValid(input)
                : input.kind === "removeLiquidity"
                ? isRemoveLiquditySignatureValid(input)
                : input.kind === "buyTokens"
                ? isBuyTokensSignatureValid(input)
                : isSellTokensSignatureValid(input)

        const buffer = new ArrayBuffer(1)
        const array = new Uint8Array(buffer)
        array[0] = isValid ? 1 : 0

        stdout.write(array)
    })
}

main()
