import { stdin, stdout } from "node:process"
import { parseInput } from "./parse"
import { continueMerge, startMerge } from "./prove"
import { saveMergedProofToDisk } from "./utils"
import { proofGenerator } from "nacho-proof-generator"

const proofsPath = process.env.NACHO_PROOFS_PATH

if (proofsPath === undefined) {
    process.exit(1)
}

await proofGenerator.compile()
stdout.write(new Uint8Array(new ArrayBuffer(1)))

stdin.on("data", async (chunk) => {
    const buffer = new ArrayBuffer(1)
    const array = new Uint8Array(buffer)
    let isSuccess = false

    try {
        const input = parseInput(chunk)

        const proof =
            input.kind === "StartMerge"
                ? await startMerge(input, proofsPath)
                : input.kind === "ContinueMerge"
                ? await continueMerge(input, proofsPath)
                : null

        if (proof) {
            saveMergedProofToDisk(proofsPath, proof)
            isSuccess = true
        }
    } catch {}

    array[0] = Number(isSuccess)
    stdout.write(array)
})
