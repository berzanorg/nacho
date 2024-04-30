import { ZkProof } from "nacho-proof-generator"
import { readFile, writeFile } from "node:fs/promises"

export const readProofFromDisk = async (proofsPath: string, index: bigint): Promise<ZkProof> => {
    const filePath = `${proofsPath}/${index}`
    const file = await readFile(filePath, { encoding: "ascii" })
    const jsonProof = ZkProof.fromJSON(JSON.parse(file))
    return jsonProof
}

export const saveMergedProofToDisk = async (proofsPath: string, proof: ZkProof): Promise<void> => {
    const filePath = `${proofsPath}/merged`
    await writeFile(filePath, JSON.stringify(proof.toJSON()), { encoding: "ascii" })
}

export const readMergedProofFromDisk = async (proofsPath: string): Promise<ZkProof> => {
    const filePath = `${proofsPath}/merged`
    const file = await readFile(filePath, { encoding: "ascii" })
    const jsonProof = ZkProof.fromJSON(JSON.parse(file))
    return jsonProof
}
