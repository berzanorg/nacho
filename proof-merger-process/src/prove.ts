import { proofGenerator } from "nacho-proof-generator"
import { ContinueMerge, StartMerge } from "./input"
import { readMergedProofFromDisk, readProofFromDisk } from "./utils"

export const startMerge = async (params: StartMerge, proofDbPath: string) => {
    const firstProof = await readProofFromDisk(proofDbPath, BigInt(params.proof_index))
    const secondProof = await readProofFromDisk(proofDbPath, BigInt(params.proof_index + 1))

    const proof = await proofGenerator.mergeProofs(firstProof.publicInput, firstProof, secondProof)

    return proof
}

export const continueMerge = async (params: ContinueMerge, proofDbPath: string) => {
    const firstProof = await readMergedProofFromDisk(proofDbPath)
    const secondProof = await readProofFromDisk(proofDbPath, BigInt(params.proof_index))

    const proof = await proofGenerator.mergeProofs(firstProof.publicInput, firstProof, secondProof)

    return proof
}
