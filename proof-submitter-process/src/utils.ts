import { ZkProof } from "nacho-proof-generator"
import { readFile } from "node:fs/promises"

export const readMergedProofFromDisk = async (proofsPath: string): Promise<ZkProof> => {
    const filePath = `${proofsPath}/merged`
    const file = await readFile(filePath, { encoding: "ascii" })
    const jsonProof = ZkProof.fromJSON(JSON.parse(file))
    return jsonProof
}
