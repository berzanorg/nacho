import { stdin, stdout } from "node:process"
import { parseInput } from "./parse.js"
import { mergeProofs } from "./prove.js"
import { saveMergedProofToDisk } from "./utils.js"
import { proofGenerator } from "nacho-proof-generator"

const main = async () => {
    const proofDbPath = process.argv.at(2)

    if (proofDbPath === undefined) {
        process.exit(1)
    }

    await proofGenerator.compile()
    stdout.write(new Uint8Array(new ArrayBuffer(1)))

    stdin.on("data", async (chunk) => {
        try {
            const input = parseInput(chunk)

            const proof =
                input.kind === "MergeProofs" ? await mergeProofs(input, proofDbPath) : null

            if (input.kind === "MergeProofs") {
                await saveMergedProofToDisk(proofDbPath, proof!)
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
}

main()
