import { stdin, stdout } from "node:process"
import { parseInput } from "./parse.js"
import {
    createGenesis,
    mergeProofs,
    depositTokens,
    makeBurnTokens,
    makeCreatePool,
    makeProvideLiquidity,
    makeRemoveLiquidity,
    makeBuyTokens,
    makeSellTokens,
} from "./prove.js"
import { saveMergedProofToDisk, saveProofToDisk } from "./utils.js"
import { proofGenerator } from "nacho-proof-generator"

const main = async () => {
    const proofDbPath = process.argv.at(2)

    if (proofDbPath === undefined) {
        process.exit(1)
    }

    await proofGenerator.compile()
    stdout.write(new Uint8Array(new ArrayBuffer(0)))

    stdin.on("data", async (chunk) => {
        const input = parseInput(chunk)

        const proof =
            input.kind === "CreateGenesis"
                ? await createGenesis(input)
                : input.kind === "MergeProofs"
                ? await mergeProofs(input, proofDbPath)
                : input.kind === "DepositTokens"
                ? await depositTokens(input, proofDbPath)
                : input.kind === "BurnTokens"
                ? await makeBurnTokens(input, proofDbPath)
                : input.kind === "CreatePool"
                ? await makeCreatePool(input, proofDbPath)
                : input.kind === "ProvideLiquidity"
                ? await makeProvideLiquidity(input, proofDbPath)
                : input.kind === "RemoveLiquidity"
                ? await makeRemoveLiquidity(input, proofDbPath)
                : input.kind === "BuyTokens"
                ? await makeBuyTokens(input, proofDbPath)
                : input.kind === "SellTokens"
                ? await makeSellTokens(input, proofDbPath)
                : null

        if (input.kind === "CreateGenesis") {
            await saveProofToDisk(proofDbPath, 0n, proof!)
        } else if (input.kind === "MergeProofs") {
            await saveMergedProofToDisk(proofDbPath, proof!)
        } else if (
            input.kind === "DepositTokens" ||
            input.kind === "BurnTokens" ||
            input.kind === "CreatePool" ||
            input.kind === "ProvideLiquidity" ||
            input.kind === "RemoveLiquidity" ||
            input.kind === "BuyTokens" ||
            input.kind === "SellTokens"
        ) {
            await saveProofToDisk(proofDbPath, input.earlier_proof_index + 1n, proof!)
        }

        const buffer = new ArrayBuffer(1)
        const array = new Uint8Array(buffer)

        const isSuccess = proof === null ? 0 : 1
        array[0] = isSuccess

        stdout.write(array)
    })
}

main()
