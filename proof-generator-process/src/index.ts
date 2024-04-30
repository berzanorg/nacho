import { stdin, stdout } from "node:process"
import { parseInput } from "./parse"
import {
    createGenesis,
    depositTokens,
    makeBurnTokens,
    makeCreatePool,
    makeProvideLiquidity,
    makeRemoveLiquidity,
    makeBuyTokens,
    makeSellTokens,
} from "./prove"
import { saveProofToDisk } from "./utils"
import { proofGenerator } from "nacho-proof-generator"

const proofsPath = process.env.NACHO_PROOFS_PATH

if (proofsPath === undefined) {
    process.exit(1)
}

await proofGenerator.compile()
stdout.write(new Uint8Array(new ArrayBuffer(1)))

stdin.on("data", async (chunk) => {
    try {
        const input = parseInput(chunk)

        const proof =
            input.kind === "CreateGenesis"
                ? await createGenesis(input)
                : input.kind === "DepositTokens"
                ? await depositTokens(input, proofsPath)
                : input.kind === "BurnTokens"
                ? await makeBurnTokens(input, proofsPath)
                : input.kind === "CreatePool"
                ? await makeCreatePool(input, proofsPath)
                : input.kind === "ProvideLiquidity"
                ? await makeProvideLiquidity(input, proofsPath)
                : input.kind === "RemoveLiquidity"
                ? await makeRemoveLiquidity(input, proofsPath)
                : input.kind === "BuyTokens"
                ? await makeBuyTokens(input, proofsPath)
                : input.kind === "SellTokens"
                ? await makeSellTokens(input, proofsPath)
                : null

        if (input.kind === "CreateGenesis") {
            await saveProofToDisk(proofsPath, 0n, proof!)
        } else if (
            input.kind === "DepositTokens" ||
            input.kind === "BurnTokens" ||
            input.kind === "CreatePool" ||
            input.kind === "ProvideLiquidity" ||
            input.kind === "RemoveLiquidity" ||
            input.kind === "BuyTokens" ||
            input.kind === "SellTokens"
        ) {
            await saveProofToDisk(proofsPath, input.earlier_proof_index + 1n, proof!)
        }

        const buffer = new ArrayBuffer(1)
        const array = new Uint8Array(buffer)

        const isSuccess = proof === null ? 0 : 1
        array[0] = isSuccess

        stdout.write(array)
    } catch (error) {
        const buffer = new ArrayBuffer(1)
        const array = new Uint8Array(buffer)

        const isSuccess = 0 // false
        array[0] = isSuccess

        stdout.write(array)
    }
})
