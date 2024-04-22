import { ZkProof } from "nacho-proof-generator"
import { readFile, writeFile } from "node:fs/promises"

export const readProofFromDisk = async (proofDbPath: string, index: bigint): Promise<ZkProof> => {
    const filePath = `${proofDbPath}/${index}`
    const file = await readFile(filePath, { encoding: "ascii" })
    const jsonProof = ZkProof.fromJSON(JSON.parse(file))
    return jsonProof
}

export const saveMergedProofToDisk = async (proofDbPath: string, proof: ZkProof): Promise<void> => {
    const filePath = `${proofDbPath}/merged`
    await writeFile(filePath, JSON.stringify(proof.toJSON()), { encoding: "ascii" })
}
